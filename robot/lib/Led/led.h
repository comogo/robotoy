#ifndef LED_H
#define LED_H

#include <Arduino.h>

#define FAST_BLINK_DELAY 100
#define SLOW_BLINK_DELAY 500

class Led {
  private:
    uint8_t m_pin;
    bool m_state;
    unsigned long m_lastBlink;

  public:
    Led(uint8_t pin);
    void initialize();
    void on();
    void off();
    void toggle();
    void fastBlink();
    void slowBlink();
};

#endif
