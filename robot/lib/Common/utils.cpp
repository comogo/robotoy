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

void padLeft(char *str, size_t num, char pad)
{
  size_t len = strlen(str);
  if (len < num)
  {
    memmove(str + num - len, str, len + 1);
    memset(str, pad, num - len);
  }
}

void toStringWithPadding(char *str, int value, size_t num, char pad)
{
  itoa(value, str, 10);
  padLeft(str, num, pad);
}
