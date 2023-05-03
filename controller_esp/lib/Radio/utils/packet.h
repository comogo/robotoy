#ifndef RADIO_PACKET_H
#define RADIO_PACKET_H

#include <Arduino.h>

enum PacketType
{
  TELEMETRY_DATA,
  CONTROLLER_DATA
};

struct Packet
{
  uint32_t id;
  uint8_t address;
  uint8_t type;
  uint8_t *payload;
  uint8_t size;
  uint32_t previousPacketId;
};

#endif
