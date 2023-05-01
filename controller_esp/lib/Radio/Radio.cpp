#include <Radio.h>

union floatByte {
  float f;
  u_int8_t b[4];
};

Radio::Radio(uint8_t id)
{
  mId = id;
  mSentPackageCounter = 1;
  mReceivedPackageCounter = 0;
  mLora = new LoRaClass();
}

Radio::~Radio()
{
  delete mLora;
}

void Radio::begin()
{
  SPI.begin(SCK, MISO, MOSI, SS);
  LoRa.setPins(SS, RST_LoRa, DIO0);
  mLora->begin(RADIO_FREQUENCY, true);
}

void Radio::incrementSentPackageCounter()
{
  if (mSentPackageCounter == UINT8_MAX)
  {
    mSentPackageCounter = 1;
  }
  else
  {
    mSentPackageCounter++;
  }
}

void Radio::incrementReceivedPackageCounter()
{
  if (mReceivedPackageCounter == UINT32_MAX)
  {
    mReceivedPackageCounter = 1;
  }
  else
  {
    mReceivedPackageCounter++;
  }
}

RequestType Radio::sendRequest(uint8_t destinationId, int8_t leftX, int8_t leftY, int8_t rightX, int8_t rightY, bool btnA, bool btnB)
{
  uint8_t requestTelemetry = 1;

  if (mSentPackageCounter % RADIO_TELEMETRY_INTERVAL != 0)
  {
    requestTelemetry = 0;
  }

  uint8_t buttons = 0;

  if (btnA)
  {
    buttons |= 0x01;
  }

  if (btnB)
  {
    buttons |= 0x02;
  }

  mLora->beginPacket();
  mLora->write(mId);              // SENDER ID
  mLora->write(destinationId);    // RECEIVER ID
  mLora->write(mSentPackageCounter);  // PACKAGE ID
  mLora->write(requestTelemetry); // REQUEST TELEMETRY?
  mLora->write(leftX);            // LEFT X
  mLora->write(leftY);            // LEFT Y
  mLora->write(rightX);           // RIGHT X
  mLora->write(rightY);           // RIGHT Y
  mLora->print(buttons);          // BUTTONS
  mLora->endPacket();

  incrementSentPackageCounter();

  if (requestTelemetry)
  {
    return RequestType::TELEMETRY_DATA;
  }
  else
  {
    return RequestType::CONTROLLER_DATA;
  }
}

TelemetryResponse Radio::receiveTelemetryResponse()
{
  int packetSize = mLora->parsePacket();

  TelemetryResponse response = {.valid = false};

  if (packetSize > 0)
  {
    floatByte batteryLevelUnion;
    uint8_t sender = (uint8_t)mLora->read();
    uint8_t receiver = (uint8_t)mLora->read();
    uint8_t packageId = (uint8_t)mLora->read();
    batteryLevelUnion.b[3] = (uint8_t)mLora->read();
    batteryLevelUnion.b[2] = (uint8_t)mLora->read();
    batteryLevelUnion.b[1] = (uint8_t)mLora->read();
    batteryLevelUnion.b[0] = (uint8_t)mLora->read();
    int rssi = mLora->packetRssi();
    float snr = mLora->packetSnr();

    // Read all the remaining bytes
    while (mLora->available())
    {
      mLora->read();
    }

    if (receiver != mId)
    {
      return response;
    }

    response.data.sender = sender;
    response.data.receiver = receiver;
    response.data.packageId = packageId;
    response.data.batteryLevel = batteryLevelUnion.f;
    response.data.rssi = rssi;
    response.data.snr = snr;
    response.valid = true;

    incrementReceivedPackageCounter();
  }

  return response;
}

void Radio::sendTelemetryResponse(uint8_t destinationId, float batteryLevel)
{
  floatByte batteryLevelUnion;

  batteryLevelUnion.f = batteryLevel;

  mLora->beginPacket();
  mLora->write(mId);              // SENDER ID
  mLora->write(destinationId);    // RECEIVER ID
  mLora->write(mSentPackageCounter);  // PACKAGE ID
  mLora->write(batteryLevelUnion.b[3]);
  mLora->write(batteryLevelUnion.b[2]);
  mLora->write(batteryLevelUnion.b[1]);
  mLora->write(batteryLevelUnion.b[0]);
  mLora->endPacket();

  incrementSentPackageCounter();

}

ControllerResponse Radio::receiveControllerResponse()
{
   int packetSize = mLora->parsePacket();

  ControllerResponse response = {
    .requestTelemetry = false,
    .valid = false
  };

  if (packetSize > 0)
  {
    uint8_t sender = (uint8_t) mLora->read();
    uint8_t receiver = (uint8_t) mLora->read();
    uint8_t packageId = (uint8_t) mLora->read();
    uint8_t requestTelemetry = (uint8_t) mLora->read();
    int8_t leftX = (int8_t) mLora->read();
    int8_t leftY = (int8_t) mLora->read();
    int8_t rightX = (int8_t) mLora->read();
    int8_t rightY = (int8_t) mLora->read();
    uint8_t buttons = (uint8_t) mLora->read();

    // Read all the remaining bytes
    while (mLora->available())
    {
      mLora->read();
    }

    if (receiver == mId)
    {
      response.data.sender = sender;
      response.data.receiver = receiver;
      response.data.packageId = packageId;
      response.data.buttonA = (buttons & 0x01) > 0;
      response.data.buttonB = (buttons & 0x02) > 0;
      response.data.leftStickX = leftX;
      response.data.leftStickY = leftY;
      response.data.rightStickX = rightX;
      response.data.rightStickY = rightY;
      response.requestTelemetry = requestTelemetry > 0;
      response.valid = true;

      incrementReceivedPackageCounter();
    }
  }

  return response;
}

uint8_t Radio::getSentPackageCounter(){
  return mSentPackageCounter;
}

uint32_t Radio::getReceivedPackageCounter(){
  return mReceivedPackageCounter;
}

