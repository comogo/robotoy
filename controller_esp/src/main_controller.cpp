#include <Arduino.h>
#include <freertos/FreeRTOS.h>
#include <freertos/task.h>
#include <Wire.h>
#include <oled/SSD1306Wire.h>
#include <Controller.h>
#include <Radio.h>
#include <utils.h>

#define CPU_FREQUENCY 240
#define MY_ID 0xA0
#define RECEIVER_ID 0xB0
#define RADIO_CONNECTION_TIMEOUT 500

struct TelemetryData
{
  float batteryLevel;
  int rssi;
  float snr;
};

SSD1306Wire *gDisplay;
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


void sendControllerData();
bool readData();
void handleDisplayLoop(void *parameter);
bool tryReadData(uint8_t times = 1);

void setup()
{
  setCpuFrequencyMhz(CPU_FREQUENCY);
  pinMode(LED, OUTPUT);

  // Enable Vext
  pinMode(Vext, OUTPUT);
  digitalWrite(Vext, LOW);

  gDisplay = new SSD1306Wire(0x3c, SDA_OLED, SCL_OLED, RST_OLED, GEOMETRY_128_64);
  gController = new Controller(true);
  gRadio = new Radio(MY_ID);

  // Setup Display
  gDisplay->init();
  gDisplay->flipScreenVertically();
  gDisplay->setFont(ArialMT_Plain_10);

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
  xSemaphoreGive(gControllerMutex);

  sendControllerData();

  if (tryReadData(100))
  {
    gConnected = true;
    gLastReceivedPacket = millis();
  }

  if (gConnected && millis() - gLastReceivedPacket > RADIO_CONNECTION_TIMEOUT)
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
      gDisplay->drawString(0, 0, "Connected!");

      xSemaphoreTake(gTelemetryMutex, portMAX_DELAY);
      gDisplay->drawString(0, 10, "R Bat: " + String(gTelemetryData.batteryLevel));
      gDisplay->drawString(0, 20, "RSSI: " + String(gTelemetryData.rssi));
      gDisplay->drawString(0, 30, "SNR: " + String(gTelemetryData.snr));
      xSemaphoreGive(gTelemetryMutex);
    }
    else
    {
      gDisplay->drawString(0, 0, "Disconnected!");

      xSemaphoreTake(gControllerMutex, portMAX_DELAY);
      gDisplay->drawString(0, 10, "BTN 1: " + String(gController->getButtonA()) + " BTN 2: " + String(gController->getButtonB()));
      gDisplay->drawString(0, 20, "Axis L X: " + String(gController->getLeftStickX()) + " Y: " + String(gController->getLeftStickY()));
      gDisplay->drawString(0, 30, "Axis R X: " + String(gController->getRightStickX()) + " Y: " + String(gController->getRightStickY()));
      xSemaphoreGive(gControllerMutex);
    }

    gDisplay->drawString(0, 40, "Lost pkg: " + String(gLostPacketCounter));
    gDisplay->drawString(0, 50, "in: " + String(gReceivedPacketCounter) + " out: " + String(gSentPacketCounter));

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
      Serial.println("R " + String(i) + " pid: " + String(gSentPacketCounter) + "c: " + String(i) + " d: " + String(millis() - startTime) + "ms");

      return true;
    }
    i = increment(i);

    if (millis() - startTime > 40)
    {
      stop = true;
    }
  }

  return false;
}
