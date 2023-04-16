#ifndef VOLTIMETER_H
#define VOLTIMETER_H

#include <timer.h>

#define VOLTIMETER_INTERVAL 1000
#define VOLTIMETER_R1 3.3
#define VOLTIMETER_R2 4.7

class Voltimeter
{
  private:
    int m_pin;
    float m_voltage;
    bool m_fresh;
    Timer *m_timer;

  public:
    Voltimeter(int pin);
    ~Voltimeter();
    void initialize();
    void read();
    float getVoltage();
    bool isFresh();
};

#endif
