#include <state.h>

State::State()
{
  m_state = STATE_INITIALIZING;
  m_lastStateTime = millis();
}

void State::setRunningState()
{
  m_state = STATE_RUNNING;
  m_lastStateTime = millis();
}

void State::setSetupState()
{
  m_state = STATE_SETUP;
  m_lastStateTime = millis();
}

bool State::isInitializing()
{
  return m_state == STATE_INITIALIZING;
}

bool State::isRunning()
{
  return m_state == STATE_RUNNING;
}

bool State::isSetup()
{
  return m_state == STATE_SETUP;
}


bool State::bounced()
{
  return millis() - m_lastStateTime > 1000;
}
