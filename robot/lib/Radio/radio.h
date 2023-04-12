#ifndef RADIO_H
#define RADIO_H

#include <SPI.h>
#include <RF24.h>
#include <timer.h>

#define RADIO_CHANNEL 125
#define RADIO_PAYLOAD_SIZE 13
#define RADIO_TIMEOUT 1000

class Radio
{
private:
  RF24 m_rf24;
  uint8_t m_address[6];
  uint8_t m_channel;
  bool m_connected;
  bool m_initialized;
  Timer m_timeout;

public:
  Radio(int ce_pin, int csn_pin, int channel, uint8_t *address);
  void initialize();
  bool available();
  void read(void *payload);
  bool is_connected();
  bool is_initialized();
};

#endif
