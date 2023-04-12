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
    Timer m_timer;
    float m_voltage;
    bool m_fresh;

  public:
    Voltimeter(int pin);
    void initialize();
    void read();
    float getVoltage();
    bool isFresh();
};

#endif
