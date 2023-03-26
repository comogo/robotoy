#include <led.h>

Led::Led(uint8_t pin) {
  m_pin = pin;
};

void Led::initialize() {
  m_state = false;
  m_lastBlink = 0;
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
  if (millis() - m_lastBlink > FAST_BLINK_DELAY) {
    toggle();
    m_lastBlink = millis();
  }
};

void Led::slowBlink() {
  if (millis() - m_lastBlink > SLOW_BLINK_DELAY) {
    toggle();
    m_lastBlink = millis();
  }
};
