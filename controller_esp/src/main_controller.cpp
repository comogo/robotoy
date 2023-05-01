#include <Arduino.h>
#include <freertos/FreeRTOS.h>
#include <freertos/task.h>
#include <Wire.h>
#include <oled/SSD1306Wire.h>
#include <Controller.h>
#include <Radio.h>

#define CPU_FREQUENCY 240
#define MY_ID 0xA0
#define RECEIVER_ID 0xB0

SSD1306Wire *gDisplay;
Controller *gController;
Radio *gRadio;
TelemetryData gResponse;
SemaphoreHandle_t gResponseMutex = xSemaphoreCreateMutex();
SemaphoreHandle_t gControllerMutex = xSemaphoreCreateMutex();
uint32_t gLostPackageCounter;
bool gConnected;
bool gWaitingResponse;
unsigned long gLastReceivedPackage;
unsigned long gLastTelemetryRequestedAt;


bool readData();
void incrementLostPackageCounter();
void handleDisplayLoop(void *parameter);

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
  gLostPackageCounter = 0;
  gConnected = false;
  gWaitingResponse = false;
  gLastReceivedPackage = 0;
  gLastTelemetryRequestedAt = 0;

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
  if (gWaitingResponse && millis() - gLastTelemetryRequestedAt > RADIO_TELEMETRY_TIMEOUT)
  {
    gWaitingResponse = false;
    incrementLostPackageCounter();
  }

  if (gWaitingResponse)
  {
    if (readData())
    {
      gConnected = true;
      gLastReceivedPackage = millis();
      gWaitingResponse = false;
      gLostPackageCounter = 0;
    }
  } else {
    xSemaphoreTake(gControllerMutex, portMAX_DELAY);
    gController->read();
    xSemaphoreGive(gControllerMutex);

    RequestType requestType = gRadio->sendRequest(RECEIVER_ID, gController->getLeftStickX(), gController->getLeftStickY(), gController->getRightStickX(), gController->getRightStickY(), gController->getButtonA(), gController->getButtonB());

    if (requestType == RequestType::TELEMETRY_DATA)
    {
      gLastTelemetryRequestedAt = millis();
      gWaitingResponse = true;
    }
  }

  if (gConnected && millis() - gLastReceivedPackage > RADIO_CONNECTION_TIMEOUT)
  {
    gConnected = false;
  }
}

void handleDisplayLoop(void *parameter)
{
  while(true)
  {
    gDisplay->clear();

    if (gConnected)
    {
      gDisplay->drawString(0, 0, "Connected!");

      xSemaphoreTake(gResponseMutex, portMAX_DELAY);
      gDisplay->drawString(0, 10, "R Bat: " + String(gResponse.batteryLevel));
      gDisplay->drawString(0, 20, "RSSI: " + String(gResponse.rssi));
      gDisplay->drawString(0, 30, "SNR: " + String(gResponse.snr));
      xSemaphoreGive(gResponseMutex);
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

    gDisplay->drawString(0, 40, "Lost pkg: " + String(gLostPackageCounter));

    gDisplay->display();
    delay(50);
  }
}

void incrementLostPackageCounter()
{
  if (gLostPackageCounter == UINT32_MAX)
  {
    gLostPackageCounter = 0;
  }
  else
  {
    gLostPackageCounter++;
  }
}

bool readData()
{
  TelemetryResponse response = gRadio->receiveTelemetryResponse();

  if (response.valid)
  {
    xSemaphoreTake(gResponseMutex, portMAX_DELAY);
    gResponse = response.data;
    xSemaphoreGive(gResponseMutex);
  }

  return response.valid;
}
