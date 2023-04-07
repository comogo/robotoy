#ifndef STATE_H
#define STATE_H

#include <Arduino.h>

#define STATE_INITIALIZING 0
#define STATE_RUNNING 1
#define STATE_SETUP 2

class State
{
  private:
    uint8_t m_state;
    unsigned long m_lastStateTime;

  public:
    State();
    void setRunningState();
    void setSetupState();
    bool isInitializing();
    bool isRunning();
    bool isSetup();
    bool bounced();
};

#endif
