#include <timer.h>
#include <Arduino.h>

Timer::Timer(unsigned long interval)
{
  m_interval = interval;
  m_lastTime = 0;
  m_running = false;
}

void Timer::setInterval(unsigned long interval)
{
  m_interval = interval;
}

void Timer::start()
{
  m_running = true;
}

void Timer::stop()
{
  m_running = false;
}

bool Timer::isRunning()
{
  return m_running;
}

bool Timer::expired()
{
  return expired(true);
}

bool Timer::expired(bool autoReset)
{
  if (!m_running)
  {
    return false;
  }

  unsigned long currentTime = millis();

  if (currentTime - m_lastTime >= m_interval)
  {
    if (autoReset)
    {
      reset(currentTime);
    }
    return true;
  }
  return false;
}

void Timer::reset(unsigned long currentTime)
{
  m_lastTime = currentTime;
}

void Timer::reset()
{
  reset(millis());
}
