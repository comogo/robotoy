#ifndef TIMER_H
#define TIMER_H

class Timer
{
  private:
    unsigned long m_interval;
    unsigned long m_lastTime;
    bool m_running;

  public:
    Timer(unsigned long interval);
    void setInterval(unsigned long interval);
    void start();
    void stop();
    void reset();
    void reset(unsigned long currentTime);
    bool isRunning();
    bool expired();
    bool expired(bool autoReset);
};
#endif
