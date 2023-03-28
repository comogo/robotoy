#include <Arduino.h>
#include <Servo.h>
#include <radio.h>
#include <motor.h>
#include <controller.h>
#include <led.h>

/*
  PWM pins: 3, 5, 6, 9, 10, 11
*/

#define MOTOR_STANDBY 2
#define MOTOR_PWM 3 // PWM
#define MOTOR_FORWARD 4
#define MOTOR_BACKWARD 5
#define LED_PIN 6
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
Led led(LED_PIN);
Servo servo;
Controller controller;
uint8_t payload[13];
uint8_t speed = 0;
uint8_t lastSpeed = speed;
int rotation = CENTER_ROTATION;
int lastRotation = 0;

void setup()
{
  led.initialize();
  led.on();
  radio.initialize();
  motor.initialize();
  servo.attach(SERVO);
  servo.write(rotation);
  delay(1000);
  led.off();
}

int handle_rotation(int16_t value, int lastValue) {
  int prepared_value = map(value, -32768, 32767, MAX_ROTATION, MIN_ROTATION);

  if (prepared_value != lastValue) {
    servo.write(prepared_value);
  }

  return prepared_value;
    }

uint16_t handle_direction(uint16_t forward_speed, uint16_t backward_speed, uint16_t lastSpeed) {
  uint16_t speed = 0;

  if (backward_speed != 0) {
        motor.setDirection(MOTOR_DIRECTION_BACKWARD);
    speed = map(backward_speed, 0, 32768, 0, MAX_SPEED);
      }

  if (forward_speed != 0) {
        motor.setDirection(MOTOR_DIRECTION_FORWARD);
    speed = map(forward_speed, 0, 32768, 0, MAX_SPEED);
      }

  if (speed != lastSpeed) {
      motor.setSpeed(speed);
    }

  return speed;
}

void loop() {
  if (radio.available()) {
    led.fastBlink();
    radio.read(&payload);
    controller.load_state_from_payload(payload);

    lastRotation = handle_rotation(controller.getYaw(), lastRotation);
    lastSpeed = handle_direction(controller.getR2(), controller.getL2(), lastSpeed);
  } else {
    led.slowBlink();
  }
}
