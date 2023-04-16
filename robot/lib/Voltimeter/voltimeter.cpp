#include <voltimeter.h>
#include <Arduino.h>

Voltimeter::Voltimeter(int pin)
{
  m_pin = pin;
  m_timer = new Timer(VOLTIMETER_INTERVAL);
  m_voltage = 0;
  m_fresh = false;
}

Voltimeter::~Voltimeter()
{
  delete m_timer;
}

void Voltimeter::initialize()
{
  pinMode(m_pin, INPUT);
  m_timer->start();
}

// Apply the voltage divider formula to get the voltage
// Vin = (Vout * (R1 + R2)) / R2
void Voltimeter::read()
{
  if (m_timer->expired())
  {
    // 5V is the max voltage of the Arduino
    // 1024 is the max value of analogRead
    // resolution is 5V / 1024 = 0.0049V
    float readVolts = analogRead(m_pin) * 0.0049;

    m_voltage = (readVolts * (VOLTIMETER_R1 + VOLTIMETER_R2)) / VOLTIMETER_R2;
    m_fresh = true;
  }
  else
  {
    m_fresh = false;
  }
}

float Voltimeter::getVoltage()
{
  return m_voltage;
}

bool Voltimeter::isFresh()
{
  return m_fresh;
}
