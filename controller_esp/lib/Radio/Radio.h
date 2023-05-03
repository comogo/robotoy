#ifndef RADIO_H
#define RADIO_H

#include <Arduino.h>
#include <SPI.h>
#include <lora/LoRa.h>
#include <utils/packet.h>

#define RADIO_FREQUENCY 915E6

class Radio
{
private:
  LoRaClass *mLora;
  uint8_t mId;
  uint32_t mSentPacketCounter;
  uint32_t mReceivedPacketCounter;
  uint32_t mLastPacketIdReceived;
  uint32_t mLastPacketIdSent;

public:
  Radio(uint8_t id);
  ~Radio();
  void begin();
  void sendPacket(Packet packet, bool reusePacketId = false);
  bool receivePacket(Packet *data);
  uint32_t getSentPacketCounter();
  uint32_t getReceivedPacketCounter();
  int getRSSI();
  float getSNR();
};

#endif
