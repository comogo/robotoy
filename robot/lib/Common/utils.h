#ifndef UTILS_H
#define UTILS_H

#include <Arduino.h>

// Apply the belzier cubic curve to the value between the range of -128 and 127
int belzier(int value, uint8_t power);

// Pad the string with the given character to the given length
void padLeft(char *str, size_t num, char pad);

// Convert the int value to a string and pad it with the given character
void toStringWithPadding(char *str, int value, size_t num, char pad);

#endif
