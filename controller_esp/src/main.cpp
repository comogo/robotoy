#include <Arduino.h>
#include <freertos/FreeRTOS.h>
#include <freertos/task.h>
#include <Wire.h>
#include <SPI.h>
#include <oled/SSD1306Wire.h>
#include <lora/LoRa.h>
#include <Controller.h>

#define CPU_FREQUENCY 240

#define RADIO_FREQUENCY 915E6
#define RADIO_ID 0xC1
#define RADIO_CONNECTION_TIMEOUT 500 // 500ms
#define RECEIVER_ID 0xB1
#define TELEMETRY_INTERVAL 32 // 32 packages
#define PACKAGE_TYPE_CONTROLLER_DATA 0x01

struct Response
{
  int sender;
  int receiver;
  int packageId;
  int batteryLevel;
  int rssi;
  float snr;
};

SSD1306Wire gDisplay = SSD1306Wire(0x3c, SDA_OLED, SCL_OLED, RST_OLED, GEOMETRY_128_64);
LoRaClass gLora = LoRaClass();
Controller *gController;
Response gResponse = {};
bool gLedState = true;
int gPackageCounter = 1;
bool gConnected = false;
unsigned long gLastReceivedPackage = 0;
SemaphoreHandle_t gResponseMutex = xSemaphoreCreateMutex();
SemaphoreHandle_t gControllerMutex = xSemaphoreCreateMutex();

bool showLoop = true;

void sendData(Controller *controller);
bool readData();
void displayData();
void handleDisplayLoop(void *parameter);

void setup()
{
  setCpuFrequencyMhz(CPU_FREQUENCY);
  pinMode(LED, OUTPUT);

  // Enable Vext
  pinMode(Vext,OUTPUT);
	digitalWrite(Vext, LOW);

  // Setup Display
  gDisplay.init();
  gDisplay.flipScreenVertically();
  gDisplay.setFont(ArialMT_Plain_10);

  // Setup LoRa
  SPI.begin(SCK,MISO,MOSI,SS);
  LoRa.setPins(SS,RST_LoRa,DIO0);
  gLora.begin(RADIO_FREQUENCY, true);

  Serial.begin(115200);
  delay(1000);

  // Setup Controller
  gController = new Controller(true);
  gController->calibrate();

  xTaskCreatePinnedToCore(
      handleDisplayLoop,   /* Function to implement the task */
      "handleDisplayLoop", /* Name of the task */
      10000,   /* Stack size in words */
      NULL,    /* Task input parameter */
      0,       /* Priority of the task */
      NULL,    /* Task handle. */
      PRO_CPU_NUM);      /* Core where the task should run */
}

void loop()
{
  xSemaphoreTake(gControllerMutex, portMAX_DELAY);
  gController->read();
  xSemaphoreGive(gControllerMutex);

  sendData(gController);

  if (readData()) {
    gConnected = true;
    gLastReceivedPackage = millis();
  }

  if (gConnected && millis() - gLastReceivedPackage > RADIO_CONNECTION_TIMEOUT) {
    gConnected = false;
  }
}

void handleDisplayLoop(void *parameter) {
  while(true) {
    displayData();
  }
}

void displayData() {
  gDisplay.clear();

  if (gConnected) {
    gDisplay.drawString(0, 0, "Connected!");

    xSemaphoreTake(gResponseMutex, portMAX_DELAY);
    gDisplay.drawString(0, 10, "R Bat: " + String(gResponse.batteryLevel));
    gDisplay.drawString(0, 20, "RSSI: " + String(gResponse.rssi));
    gDisplay.drawString(0, 30, "SNR: " + String(gResponse.snr));
    xSemaphoreGive(gResponseMutex);
  } else {
    gDisplay.drawString(0, 0, "Disconnected!");

    xSemaphoreTake(gControllerMutex, portMAX_DELAY);
    gDisplay.drawString(0, 10, "BTN 1: " + String(gController->getButtonA()) + " BTN 2: " + String(gController->getButtonB()));
    gDisplay.drawString(0, 20, "Axis L X: " + String(gController->getLeftStickX()) + " Y: " + String(gController->getLeftStickY()));
    gDisplay.drawString(0, 30, "Axis R X: " + String(gController->getRightStickX()) + " Y: " + String(gController->getRightStickY()));
    xSemaphoreGive(gControllerMutex);
  }

  gDisplay.display();
}

void sendData(Controller *controller)
{
  bool requestTelemetry = gPackageCounter % TELEMETRY_INTERVAL == 0;

  gLora.beginPacket();
  gLora.write(RADIO_ID);                     // SENDER ID
  gLora.write(RECEIVER_ID);                  // RECEIVER ID
  gLora.write(gPackageCounter);               // PACKAGE ID
  gLora.write(requestTelemetry);             // REQUEST TELEMETRY
  gLora.write(PACKAGE_TYPE_CONTROLLER_DATA); // PACKAGE TYPE
  gLora.print(controller->getLeftStickX());  // LEFT X
  gLora.print(controller->getLeftStickY());  // LEFT Y
  gLora.print(controller->getRightStickX()); // RIGHT X
  gLora.print(controller->getRightStickY()); // RIGHT Y
  gLora.print(controller->getButtonA());     // BUTTON A
  gLora.print(controller->getButtonB());     // BUTTON B
  gLora.print(controller->getButtonC());     // BUTTON C
  gLora.endPacket();

  if (gPackageCounter == 255) {
    gPackageCounter = 1;
  } else {
    gPackageCounter++;
  }
}

bool readData()
{
  int packetSize = gLora.parsePacket();

  if (packetSize <= 0) { return false; }

  int sender = gLora.read();
  int receiver = gLora.read();
  int packageId = gLora.read();
  int batteryLevel = gLora.parseFloat();
  int rssi = gLora.packetRssi();
  float snr = gLora.packetSnr();

  // Read all the remaining bytes
  while (gLora.available()) { gLora.read(); }

  xSemaphoreTake(gResponseMutex, portMAX_DELAY);
  gResponse.sender = sender;
  gResponse.receiver = receiver;
  gResponse.packageId = packageId;
  gResponse.batteryLevel = batteryLevel;
  gResponse.rssi = rssi;
  gResponse.snr = snr;
  xSemaphoreGive(gResponseMutex);

  return true;
}
