#include <led.h>

Led::Led(uint8_t pin) {
  m_pin = pin;
  m_fast_timer = Timer(FAST_BLINK_DELAY);
  m_slow_timer = Timer(SLOW_BLINK_DELAY);
};

void Led::initialize() {
  m_state = false;
  m_fast_timer.start();
  m_slow_timer.start();
  pinMode(m_pin, OUTPUT);
  off();
};

void Led::on() {
  if (m_state) { return; }

  digitalWrite(m_pin, HIGH);
  m_state = true;
};

void Led::off() {
  if (!m_state) { return; }

  digitalWrite(m_pin, LOW);
  m_state = false;
};

void Led::toggle() {
  if (m_state) {
    off();
  } else {
    on();
  }
};

void Led::fastBlink() {
  if (m_fast_timer.isReady(false)) {
    toggle();
    m_fast_timer.reset();
  }
};

void Led::slowBlink() {
  if (m_slow_timer.isReady(false)) {
    toggle();
    m_slow_timer.reset();
  }
};
