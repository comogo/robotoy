#ifndef LED_H
#define LED_H

#include <Arduino.h>
#include <timer.h>

#define FAST_BLINK_DELAY 100
#define SLOW_BLINK_DELAY 500

class Led {
  private:
    uint8_t m_pin;
    bool m_state;
    Timer *m_fast_timer;
    Timer *m_slow_timer;

  public:
    Led(uint8_t pin);
    ~Led();
    void initialize();
    void on();
    void off();
    void toggle();
    void fastBlink();
    void slowBlink();
};

#endif
