#ifndef RESPONSES_H
#define RESPONSES_H

#include <Arduino.h>

struct TelemetryData
{
  int sender;
  int receiver;
  uint32_t packageId;
  float batteryLevel;
  int rssi;
  float snr;
};

struct ControllerData
{
  int8_t sender;
  int8_t receiver;
  uint32_t packageId;
  int8_t leftStickX;
  int8_t leftStickY;
  int8_t rightStickX;
  int8_t rightStickY;
  bool buttonA;
  bool buttonB;
};

struct TelemetryResponse
{
  TelemetryData data;
  bool valid;
};

struct ControllerResponse
{
  ControllerData data;
  bool requestTelemetry;
  bool valid;
};


#endif
