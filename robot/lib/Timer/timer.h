#ifndef TIMER_H
#define TIMER_H

#define TIMER_DEFAULT_INTERVAL 1000

class Timer
{
  private:
    unsigned long m_interval;
    unsigned long m_lastTime;
    bool m_running;

  public:
    Timer(unsigned long interval);
    Timer();
    void setInterval(unsigned long interval);
    void start();
    void stop();
    void reset();
    bool isRunning();
    bool isReady();
    bool isReady(bool autoReset);
    void tryRun(void (*callback)());
};
#endif
