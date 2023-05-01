#include <Arduino.h>
#include <freertos/FreeRTOS.h>
#include <freertos/task.h>
#include <Wire.h>
#include <oled/SSD1306Wire.h>
#include <Radio.h>

#define CPU_FREQUENCY 240
#define CONTROLLER_ID 0xA0
#define MY_ID 0xB0

uint8_t gId;
ControllerData gControllerState;
SSD1306Wire *gDisplay;
Radio *gRadio;

void handleDisplayLoop(void *parameter)
{
  while(true)
  {
    gDisplay->clear();
    gDisplay->drawString(0, 10, "Pkg counter: out:" + String(gRadio->getSentPackageCounter()) + " in:" + String(gRadio->getReceivedPackageCounter()));
    gDisplay->drawString(0, 20, "L Stick X: " + String(gControllerState.leftStickX) + " Y: " + String(gControllerState.leftStickY));
    gDisplay->drawString(0, 30, "R Stick X: " + String(gControllerState.rightStickX) + " Y: " + String(gControllerState.rightStickY));
    gDisplay->drawString(0, 40, "Buttons: A: " + String(gControllerState.buttonA) + " B: " + String(gControllerState.buttonB));
    gDisplay->display();
    delay(10);
  }
}

void setup()
{
  setCpuFrequencyMhz(CPU_FREQUENCY);
  pinMode(LED, OUTPUT);

  // Disable Vext
  pinMode(Vext, OUTPUT);
  digitalWrite(Vext, HIGH);

  gDisplay = new SSD1306Wire(0x3c, SDA_OLED, SCL_OLED, RST_OLED, GEOMETRY_128_64);
  gRadio = new Radio(MY_ID);
  gControllerState = ControllerData{
    .leftStickX = 0,
    .leftStickY = 0,
    .rightStickX = 0,
    .rightStickY = 0,
    .buttonA = false,
    .buttonB = false
  };

  // Setup Display
  gDisplay->init();
  gDisplay->flipScreenVertically();
  gDisplay->setFont(ArialMT_Plain_10);

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
  ControllerResponse response = gRadio->receiveControllerResponse();

  if (response.valid)
  {
    gControllerState = response.data;

    if (response.requestTelemetry)
    {
      delay(20);
      gRadio->sendTelemetryResponse(CONTROLLER_ID, 8.0);
    }
  }
}
