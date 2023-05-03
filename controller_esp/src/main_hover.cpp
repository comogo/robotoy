#include <Arduino.h>
#include <freertos/FreeRTOS.h>
#include <freertos/task.h>
#include <Wire.h>
#include <Display.h>
#include <Radio.h>
#include <utils.h>

#define CPU_FREQUENCY 240
#define TELEMETRY_PACKET_RATE 16
#define CONTROLLER_ID 0xA0
#define MY_ID 0xB0

struct ControllerState
{
  int8_t leftStickX;
  int8_t leftStickY;
  int8_t rightStickX;
  int8_t rightStickY;
  bool buttonA;
  bool buttonB;
};

Display *gDisplay;
Radio *gRadio;
ControllerState gControllerState = {
  .leftStickX = 0,
  .leftStickY = 0,
  .rightStickX = 0,
  .rightStickY = 0,
  .buttonA = false,
  .buttonB = false
};
float gBatteryLevel;
uint32_t gSentPacketCounter = 0;
uint32_t gReceivedPacketCounter = 0;
uint32_t gLostPacketCounter = 0;
uint32_t gLastPacketId = 0;

void handleDisplayLoop(void *parameter);
void displayData();
void sendTelemetry();
bool receiveControllerData();

void setup()
{
  setCpuFrequencyMhz(CPU_FREQUENCY);
  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, LOW);

  // Disable Vext
  pinMode(Vext, OUTPUT);
  digitalWrite(Vext, HIGH);

  Wire.begin(SDA_OLED, SCL_OLED, 700000U);
  gDisplay = new Display();
  gRadio = new Radio(MY_ID);

  // Setup Display
  gDisplay->begin();

  // Setup Radio
  gRadio->begin();

  Serial.begin(115200);
  delay(1000);

  xTaskCreatePinnedToCore(
      handleDisplayLoop,   /* Function to implement the task */
      "handleDisplayLoop", /* Name of the task */
      10000,               /* Stack size in words */
      NULL,                /* Task input parameter */
      10,                   /* Priority of the task */
      NULL,                /* Task handle. */
      PRO_CPU_NUM);        /* Core where the task should run */
}

void loop()
{
  if (receiveControllerData())
  {
    gReceivedPacketCounter = gRadio->getReceivedPacketCounter();

    if (gLastPacketId % TELEMETRY_PACKET_RATE == 0)
    {
      sendTelemetry();
      gSentPacketCounter = gRadio->getSentPacketCounter();
    }
  }

  // displayData();
}

void handleDisplayLoop(void *parameter)
{
  while(true)
  {
    displayData();
    delay(50);
  }
}

void displayData()
{
  gDisplay->clear();
  gDisplay->drawString(0, 10, "Lost pkg:" + String(gLostPacketCounter));
  gDisplay->drawString(0, 20, "L Stick X: " + String(gControllerState.leftStickX) + " Y: " + String(gControllerState.leftStickY));
  gDisplay->drawString(0, 30, "R Stick X: " + String(gControllerState.rightStickX) + " Y: " + String(gControllerState.rightStickY));
  gDisplay->drawString(0, 40, "Buttons: A: " + String(gControllerState.buttonA) + " B: " + String(gControllerState.buttonB));
  gDisplay->display();
}

void sendTelemetry()
{
  UFloatByte_t uBatteryLevel;
  uBatteryLevel.f = gBatteryLevel;

  uint8_t payload[4] = {
    uBatteryLevel.b[3],
    uBatteryLevel.b[2],
    uBatteryLevel.b[1],
    uBatteryLevel.b[0]
  };

  Packet packet = Packet{
    .address = CONTROLLER_ID,
    .type = PacketType::TELEMETRY_DATA,
    .payload = payload,
    .size = sizeof(payload)
  };

  gRadio->sendPacket(packet);
  gRadio->sendPacket(packet, true);
  gRadio->sendPacket(packet, true);
}

bool receiveControllerData()
{
  uint8_t payload[200] = {0};
  Packet packet;
  packet.payload = payload;

  if (gRadio->receivePacket(&packet))
  {
    if (packet.type == PacketType::CONTROLLER_DATA)
    {
      gControllerState.leftStickX = packet.payload[0];
      gControllerState.leftStickY = packet.payload[1];
      gControllerState.rightStickX = packet.payload[2];
      gControllerState.rightStickY = packet.payload[3];
      gControllerState.buttonA = (packet.payload[4] & 0x01) > 0;
      gControllerState.buttonB = (packet.payload[4] & 0x02) > 0;
    }

    if (gLastPacketId != packet.previousPacketId)
    {
      gLostPacketCounter = increment(gLostPacketCounter);
    }

    gLastPacketId = packet.id;

    return true;
  }

  return false;
}
