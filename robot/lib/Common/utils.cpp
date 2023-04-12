#include <utils.h>

int belzier(int value, uint8_t power)
{
  if (value > 0)
  {
    return map(pow(value, 3), 0, pow(127, 3), 0, 127);
  }
  else
  {
    return map(pow(value, 3), 0, pow(-128, 3), 0, -128);
  }
}
