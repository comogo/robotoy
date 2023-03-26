#ifndef MOTOR_H
#define MOTOR_H

#define MOTOR_DIRECTION_FORWARD 1
#define MOTOR_DIRECTION_BACKWARD 2
#define MOTOR_STATE_STOPPED 0
#define MOTOR_STATE_RUNNING 1

class Motor
{
private:
  int speed;
  int direction;
  int pwm_pin;
  int forward_pin;
  int backward_pin;
  int standby_pin;
  int state;

public:
  Motor(int standby_pin, int pwm_pin, int forward_pin, int backward_pin);
  void initialize();
  void setSpeed(int newSpeed);
  void setDirection(int newDirection);
  void stop();
  bool isRunning();
};

#endif
