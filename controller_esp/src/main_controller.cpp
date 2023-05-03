#include <Arduino.h>
#include <freertos/FreeRTOS.h>
#include <freertos/task.h>
#include <Wire.h>
#include <Display.h>
#include <Controller.h>
#include <Radio.h>
#include <utils.h>

#define CPU_FREQUENCY 240
#define MY_ID 0xA0
#define RECEIVER_ID 0xB0
#define RADIO_CONNECTION_TIMEOUT 500
#define BAT_VOLTAGE_PIN GPIO_NUM_37

struct TelemetryData
{
  float batteryLevel;
  int rssi;
  float snr;
};

Display *gDisplay;
Controller *gController;
Radio *gRadio;
TelemetryData gTelemetryData;
SemaphoreHandle_t gTelemetryMutex = xSemaphoreCreateMutex();
SemaphoreHandle_t gControllerMutex = xSemaphoreCreateMutex();
bool gConnected = false;
unsigned long gLastReceivedPacket = 0;
uint32_t gLostPacketCounter = 0;
uint32_t gReceivedPacketCounter = 0;
uint32_t gSentPacketCounter = 0;
uint32_t gLastPacketId = 0;
float gBatteryVoltage = 0.0;


void sendControllerData();
bool readData();
void handleDisplayLoop(void *parameter);
bool tryReadData(uint8_t times = 1);

void setup()
{
  setCpuFrequencyMhz(CPU_FREQUENCY);
  analogSetAttenuation(ADC_11db);
  analogSetPinAttenuation(BAT_VOLTAGE_PIN, ADC_11db);
  pinMode(BAT_VOLTAGE_PIN, INPUT);
  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, LOW);

  // Disable Vext
  pinMode(Vext, OUTPUT);
  digitalWrite(Vext, HIGH);

  Wire.begin(SDA_OLED, SCL_OLED, 700000U);
  gDisplay = new Display();
  gController = new Controller(true);
  gRadio = new Radio(MY_ID);

  // Setup Display
  gDisplay->begin();
  gDisplay->flip();

  // Setup Radio
  gRadio->begin();

  Serial.begin(115200);
  delay(1000);

  // Setup Controller
  gController->calibrate();

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
  xSemaphoreTake(gControllerMutex, portMAX_DELAY);
  gController->read();
  gBatteryVoltage = (analogRead(BAT_VOLTAGE_PIN) * 0.769 + 150) / 1000.0;
  xSemaphoreGive(gControllerMutex);

  sendControllerData();

  if (tryReadData(30))
  {
    gConnected = true;
    gLastReceivedPacket = millis();
    gLostPacketCounter = 0;
  }

  if (gConnected && gLostPacketCounter > 3)
  {
    gConnected = false;
  }

  gSentPacketCounter = gRadio->getSentPacketCounter();
  gReceivedPacketCounter = gRadio->getReceivedPacketCounter();
}

void handleDisplayLoop(void *parameter)
{
  while(true)
  {
    gDisplay->clear();

    if (gConnected)
    {
      xSemaphoreTake(gTelemetryMutex, portMAX_DELAY);
      gDisplay->drawString(0, 0, "Bat: " + String(gBatteryVoltage) + " R Bat: " + String(gTelemetryData.batteryLevel));
      gDisplay->drawString(0, 10, "RSSI: " + String(gTelemetryData.rssi));
      gDisplay->drawString(0, 20, "SNR: " + String(gTelemetryData.snr));
      xSemaphoreGive(gTelemetryMutex);
    }
    else
    {
      gDisplay->drawString(0, 0, "Bat: " + String(gBatteryVoltage));
      xSemaphoreTake(gControllerMutex, portMAX_DELAY);
      gDisplay->drawString(0, 0, "BTN 1: " + String(gController->getButtonA()) + " BTN 2: " + String(gController->getButtonB()));
      gDisplay->drawString(0, 10, "Axis L X: " + String(gController->getLeftStickX()) + " Y: " + String(gController->getLeftStickY()));
      gDisplay->drawString(0, 20, "Axis R X: " + String(gController->getRightStickX()) + " Y: " + String(gController->getRightStickY()));
      xSemaphoreGive(gControllerMutex);
    }

    gDisplay->drawString(0, 30, "Lost pkg: " + String(gLostPacketCounter));
    gDisplay->drawString(0, 40, "in: " + String(gReceivedPacketCounter) + " out: " + String(gSentPacketCounter));

    gDisplay->display();
    delay(50);
  }
}

void sendControllerData()
{
  uint8_t leftX = (uint8_t)gController->getLeftStickX();
  uint8_t leftY = (uint8_t)gController->getLeftStickY();
  uint8_t rightX = (uint8_t)gController->getRightStickX();
  uint8_t rightY = (uint8_t)gController->getRightStickY();
  uint8_t buttons = 0;

  if (gController->getButtonA())
  {
    buttons |= 0x01;
  }

  if (gController->getButtonB())
  {
    buttons |= 0x02;
  }

  uint8_t payload[5] = {
    leftX,
    leftY,
    rightX,
    rightY,
    buttons
  };

  Packet packet = {
      .address = RECEIVER_ID,
      .type = PacketType::CONTROLLER_DATA,
      .payload = payload,
      .size = 5
  };

  gRadio->sendPacket(packet);
  gRadio->sendPacket(packet, true);
  gRadio->sendPacket(packet, true);
}

bool readData()
{
  uint8_t payload[200] = {0};
  Packet packet;
  packet.payload = payload;

  if (gRadio->receivePacket(&packet))
  {
    if (packet.type == PacketType::TELEMETRY_DATA)
    {
      UFloatByte_t uBatteryLevel;
      uBatteryLevel.b[0] = packet.payload[0];
      uBatteryLevel.b[1] = packet.payload[1];
      uBatteryLevel.b[2] = packet.payload[2];
      uBatteryLevel.b[3] = packet.payload[3];

      xSemaphoreTake(gTelemetryMutex, portMAX_DELAY);
      gTelemetryData.batteryLevel = uBatteryLevel.f;
      gTelemetryData.rssi = gRadio->getRSSI();
      gTelemetryData.snr = gRadio->getSNR();
      xSemaphoreGive(gTelemetryMutex);
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

bool tryReadData(uint8_t times)
{
  bool stop = false;
  unsigned long startTime = millis();
  uint32_t i = 1;
  while(!stop)
  {
    if (readData())
    {
      return true;
    }
    i = increment(i);

    if (millis() - startTime > 30)
    {
      stop = true;
    }
  }

  return false;
}
