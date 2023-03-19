#include <motor.h>
#include <Arduino.h>

Motor::Motor(int standby_pin_num, int pwm_pin_num, int forward_pin_num, int backward_pin_num) {
  standby_pin = standby_pin_num;
  pwm_pin = pwm_pin_num;
  forward_pin = forward_pin_num;
  backward_pin = backward_pin_num;
  direction = MOTOR_DIRECTION_FORWARD;
  speed = 0;
  state = MOTOR_STATE_STOPPED;
}

void Motor::initialize() {
  pinMode(standby_pin, OUTPUT);
  pinMode(pwm_pin, OUTPUT);
  pinMode(forward_pin, OUTPUT);
  pinMode(backward_pin, OUTPUT);
  digitalWrite(standby_pin, LOW);
  digitalWrite(forward_pin, LOW);
  digitalWrite(backward_pin, LOW);
  analogWrite(pwm_pin, 0);
}

void Motor::setSpeed(int newSpeed) {
  if (newSpeed < 0) {
    speed = 0;
  } else if (newSpeed > 255) {
    speed = 255;
  }

  if (speed == newSpeed) {
    return;
  }

  speed = newSpeed;
  analogWrite(pwm_pin, speed);

  if (state != MOTOR_STATE_RUNNING) {
    state = MOTOR_STATE_RUNNING;
    digitalWrite(standby_pin, HIGH);
  }
}

void Motor::setDirection(int newDirection) {
  if (newDirection != MOTOR_DIRECTION_FORWARD && newDirection != MOTOR_DIRECTION_BACKWARD) {
    direction = MOTOR_DIRECTION_FORWARD;
  }

  if (direction == newDirection) {
    return;
  }

  direction = newDirection;
  switch (direction) {
    case MOTOR_DIRECTION_FORWARD:
      digitalWrite(forward_pin, HIGH);
      digitalWrite(backward_pin, LOW);
      break;
    case MOTOR_DIRECTION_BACKWARD:
      digitalWrite(forward_pin, LOW);
      digitalWrite(backward_pin, HIGH);
      break;
  }
}

void Motor::stop() {
  if (state == MOTOR_STATE_STOPPED) {
    return;
  }

  digitalWrite(standby_pin, LOW);
  digitalWrite(forward_pin, LOW);
  digitalWrite(backward_pin, LOW);
  setSpeed(0);
  state = MOTOR_STATE_STOPPED;
}

bool Motor::isRunning() {
  return state == MOTOR_STATE_RUNNING;
}
