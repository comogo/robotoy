#include <radio.h>

Radio::Radio(int ce_pin, int csn_pin, int channel, uint8_t *address)
{
  m_channel = channel;
  m_connected = false;
  m_initialized = false;
  m_last_message_time = 0;
  memcpy(m_address, address, 6);
  m_rf24 = RF24(ce_pin, csn_pin);
}

void Radio::initialize()
{
  if (m_initialized)
  {
    return;
  }

  if (m_rf24.begin())
  {
    m_rf24.setPALevel(RF24_PA_MAX);
    m_rf24.setDataRate(RF24_250KBPS);
    m_rf24.setPayloadSize(RADIO_PAYLOAD_SIZE);
    m_rf24.setAutoAck(false);
    m_rf24.setChannel(m_channel);
    m_rf24.openReadingPipe(0, m_address);
    m_rf24.startListening();
    m_initialized = true;
  }
}

bool Radio::available()
{
  uint8_t pipe;
  bool available_data = m_rf24.available(&pipe);
  if (available_data)
  {
    m_last_message_time = millis();
    m_connected = true;
  }
  else if (millis() - m_last_message_time > RADIO_TIMEOUT)
  {
    m_connected = false;
  }

  return available_data;
}

void Radio::read(void *payload)
{
  uint8_t bytes = m_rf24.getPayloadSize();
  m_rf24.read(payload, bytes);
}

bool Radio::is_connected()
{
  return m_initialized && m_connected;
}

bool Radio::is_initialized()
{
  return m_initialized;
}
