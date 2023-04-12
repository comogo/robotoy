#ifndef UTILS_H
#define UTILS_H

#include <Arduino.h>

// Apply the belzier cubic curve to the value between the range of -128 and 127
int belzier(int value, uint8_t power);

#endif
