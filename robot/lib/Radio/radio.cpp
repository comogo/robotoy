#include <radio.h>

Radio::Radio(int ce_pin, int csn_pin, int channel, uint8_t *address)
{
  m_channel = channel;
  m_connected = false;
  m_initialized = false;
  m_connectionSpeedRate = 0;
  m_packageCounter = 0;
  m_lastPackageCounter = 0;
  m_timeout = new Timer(RADIO_TIMEOUT);
  m_packageConnectionSpeedTimer = new Timer(RADIO_CONNECTION_SPEED_MEASURE_INTERVAL);
  memcpy(m_address, address, 6);
  m_rf24 = new RF24(ce_pin, csn_pin);
}

Radio::~Radio()
{
  delete m_rf24;
  delete m_timeout;
}

void Radio::initialize()
{
  if (m_initialized)
  {
    return;
  }

  if (m_rf24->begin())
  {
    m_rf24->setPALevel(RF24_PA_MAX);
    m_rf24->setDataRate(RF24_250KBPS);
    m_rf24->setPayloadSize(RADIO_PAYLOAD_SIZE);
    m_rf24->setAutoAck(false);
    m_rf24->setChannel(m_channel);
    m_rf24->openReadingPipe(0, m_address);
    m_rf24->startListening();
    m_initialized = true;
  }

  m_timeout->start();
  m_packageConnectionSpeedTimer->start();
}

bool Radio::available()
{
  if (!m_initialized)
  {
    m_connected = false;
    return false;
  }

  uint8_t pipe;
  bool available_data = m_rf24->available(&pipe);
  if (available_data)
  {
    m_timeout->reset();
    m_connected = true;
    m_packageCounter++;
    calculateConnectionSpeed();
  }
  else if (m_connected && m_timeout->expired(false))
  {
    m_connected = false;
    m_connectionSpeedRate = 0;
    m_packageCounter = 0;
    m_lastPackageCounter = 0;
  }

  return available_data;
}

void Radio::read(void *payload)
{
  uint8_t bytes = m_rf24->getPayloadSize();
  m_rf24->read(payload, bytes);
}

bool Radio::isConnected()
{
  return isInitialized() && m_connected;
}

bool Radio::isInitialized()
{
  return m_initialized;
}

void Radio::calculateConnectionSpeed()
{
  if (m_packageConnectionSpeedTimer->expired(true))
  {
    m_connectionSpeedRate = (m_lastPackageCounter + m_packageCounter) / 2;
    m_lastPackageCounter = m_packageCounter;
    m_packageCounter = 0;
  }
}

int Radio::getConnectionSpeedRate()
{
  return m_connectionSpeedRate;
}
