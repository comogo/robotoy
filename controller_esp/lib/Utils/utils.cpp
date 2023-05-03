#include <utils.h>

uint8_t increment(uint8_t i)
{
  if (i >= UINT8_MAX)
  {
    return 0;
  }
  else
  {
    return i + 1;
  }
}

uint16_t increment(uint16_t i)
{
  if (i >= UINT16_MAX)
  {
    return 0;
  }
  else
  {
    return i + 1;
  }
}

uint32_t increment(uint32_t i)
{
  if (i >= UINT32_MAX)
  {
    return 0;
  }
  else
  {
    return i + 1;
  }
}
