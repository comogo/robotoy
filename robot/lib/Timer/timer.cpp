#include <timer.h>
#include <Arduino.h>

Timer::Timer(unsigned long interval)
{
  m_interval = interval;
  m_lastTime = 0;
  m_running = false;
}

Timer::Timer()
{
  Timer(TIMER_DEFAULT_INTERVAL);
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

bool Timer::isReady()
{
  return isReady(true);
}

bool Timer::isReady(bool autoReset)
{
  if (m_running && millis() - m_lastTime >= m_interval)
  {
    if (autoReset)
    {
      reset();
    }
    return true;
  }
  return false;
}

void Timer::tryRun(void (*callback)())
{
  if (isReady())
  {
    callback();
  }
}

void Timer::reset()
{
  m_lastTime = millis();
}
