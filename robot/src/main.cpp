#include <Arduino.h>
#include <Servo.h>
#include <radio.h>
#include <motor.h>
#include <controller.h>

/*
  PWM pins: 3, 5, 6, 9, 10, 11
*/

#define MOTOR_STANDBY 2
#define MOTOR_PWM 3 // PWM
#define MOTOR_FORWARD 4
#define MOTOR_BACKWARD 5
#define LED 6 // PWM
#define RADIO_CE 7
#define RADIO_CSN 8
#define RADIO_CHANNEL 125
#define SERVO 9 // PWM

#define MAX_SPEED 250
#define MAX_ROTATION 180
#define MIN_ROTATION 0
#define CENTER_ROTATION 90

uint8_t address[6] = "aaaaa";
Radio radio(RADIO_CE, RADIO_CSN, RADIO_CHANNEL, address);
Motor motor(MOTOR_STANDBY, MOTOR_PWM, MOTOR_FORWARD, MOTOR_BACKWARD);
Servo servo;
Controller controller;
uint8_t payload[13];
uint8_t speed = 0;
uint8_t lastSpeed = speed;
int16_t rotation = CENTER_ROTATION;
int16_t lastRotation = 0;

void setup()
{
  pinMode(LED, OUTPUT);
  analogWrite(LED, 100);

  radio.initialize();
  motor.initialize();
  servo.attach(SERVO);
  servo.write(rotation);
}

void loop()
{
  if (radio.available())
  {
    radio.read(&payload);
    controller.load_state_from_payload(payload);

    speed = 0;

    rotation = map(controller.getYaw(), -32768, 32767, MAX_ROTATION, MIN_ROTATION);

    if (rotation != lastRotation)
    {
      servo.write(rotation);
    }

    if (controller.getL2() == 0 && controller.getR2() == 0)
    {
      motor.setSpeed(0);
    }
    else
    {
      if (controller.getL2() != 0)
      {
        motor.setDirection(MOTOR_DIRECTION_BACKWARD);
        speed = map(controller.getL2(), 0, 32768, 0, MAX_SPEED);
      }

      if (controller.getR2() != 0)
      {
        motor.setDirection(MOTOR_DIRECTION_FORWARD);
        speed = map(controller.getR2(), 0, 32768, 0, MAX_SPEED);
      }

      motor.setSpeed(speed);
    }
  }

  if (lastSpeed != speed)
  {
    analogWrite(LED, speed);
  }

  lastSpeed = speed;
  lastRotation = rotation;
}
