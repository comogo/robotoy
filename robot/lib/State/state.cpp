#include <state.h>

State::State()
{
  m_state = STATE_INITIALIZING;
  m_lastStateTime = millis();
}

void State::setRunningState()
{
  if (isRunning())
  {
    return;
  }

  m_state = STATE_RUNNING;
  m_lastStateTime = millis();
}

void State::setSetupState()
{
  if (isSetup())
  {
    return;
  }

  m_state = STATE_SETUP;
  m_lastStateTime = millis();
}

void State::setDisconnectedState()
{
  if (isDisconnected())
  {
    return;
  }

  m_state = STATE_DISCONNECTED;
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

bool State::isDisconnected()
{
  return m_state == STATE_DISCONNECTED;
}

bool State::bounced()
{
  return millis() - m_lastStateTime > STATE_BOUNCE_DELAY;
}
