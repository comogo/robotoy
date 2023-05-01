#ifndef RADIO_H
#define RADIO_H

#include <Arduino.h>
#include <SPI.h>
#include <lora/LoRa.h>
#include <Responses.h>

#define RADIO_FREQUENCY 915E6
#define RADIO_TELEMETRY_TIMEOUT 1000   // 50ms
#define RADIO_TELEMETRY_INTERVAL 5  // 32 packages
#define RADIO_CONNECTION_TIMEOUT (RADIO_TELEMETRY_TIMEOUT * RADIO_TELEMETRY_INTERVAL) // 500ms
#define RADIO_PACKAGE_TYPE_CONTROLLER_DATA 0x01

enum RequestType
{
  CONTROLLER_DATA,
  TELEMETRY_DATA
};

class Radio
{
private:
  LoRaClass *mLora;
  uint8_t mId;
  uint8_t mSentPackageCounter;
  uint32_t mReceivedPackageCounter;

  void incrementSentPackageCounter();
  void incrementReceivedPackageCounter();

public:
  Radio(uint8_t id);
  ~Radio();
  void begin();

  RequestType sendRequest(uint8_t destinationId, int8_t leftX, int8_t leftY, int8_t rightX, int8_t rightY, bool btnA, bool btnB);
  TelemetryResponse receiveTelemetryResponse();
  void sendTelemetryResponse(uint8_t destinationId, float batteryLevel);
  ControllerResponse receiveControllerResponse();

  uint8_t getSentPackageCounter();
  uint32_t getReceivedPackageCounter();
};

#endif
