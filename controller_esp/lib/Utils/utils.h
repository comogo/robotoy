#ifndef MY_UTILS_H
#define MY_UTILS_H

#include <Arduino.h>

union UFloatByte_t {
  float f;
  u_int8_t b[4];
};

union UUint32Byte_t {
  uint32_t i;
  u_int8_t b[4];
};

uint8_t increment(uint8_t i);
uint16_t increment(uint16_t i);
uint32_t increment(uint32_t i);

#endif
