#ifndef STATE_H
#define STATE_H

#include <Arduino.h>

#define STATE_INITIALIZING 0
#define STATE_RUNNING 1
#define STATE_SETUP 2
#define STATE_DISCONNECTED 3
#define STATE_BOUNCE_DELAY 1000

class State
{
private:
  uint8_t m_state;
  unsigned long m_lastStateTime;

public:
  State();
  void setRunningState();
  void setSetupState();
  void setDisconnectedState();
  bool isInitializing();
  bool isRunning();
  bool isDisconnected();
  bool isSetup();
  bool bounced();
};

#endif
