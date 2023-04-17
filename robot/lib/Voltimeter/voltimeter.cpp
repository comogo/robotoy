#include <voltimeter.h>
#include <Arduino.h>

Voltimeter::Voltimeter(int vAnalogPin, int cAnalogPin)
{
  m_vpin = vAnalogPin;
  m_cpin = cAnalogPin;
  m_timer = new Timer(VOLTIMETER_INTERVAL);
  m_voltage = 0;
  m_current = 0;
  m_fresh = false;
}

Voltimeter::~Voltimeter()
{
  delete m_timer;
}

void Voltimeter::initialize()
{
  pinMode(m_vpin, INPUT);
  pinMode(m_cpin, INPUT);
  m_timer->start();
}

void Voltimeter::read()
{
  m_fresh = false;

  if (m_timer->expired())
  {
    m_voltage = ((analogRead(m_vpin) * 0.004882813) * (VOLTIMETER_R1 + VOLTIMETER_R2)) / VOLTIMETER_R2;
    m_current = ((2.5 - (analogRead(m_cpin)) * 0.004882813) * 1000) / 66;
    m_fresh = true;
  }
}

float Voltimeter::getVoltage()
{
  return m_voltage;
}

float Voltimeter::getCurrent()
{
  return m_current;
}

bool Voltimeter::isFresh()
{
  return m_fresh;
}
