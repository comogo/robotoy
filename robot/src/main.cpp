#include <Arduino.h>
#include <SPI.h>
#include "RF24.h"
#include <Servo.h>
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
#define SERVO 9 // PWM


RF24 radio(RADIO_CE, RADIO_CSN);
Motor motor(MOTOR_STANDBY, MOTOR_PWM, MOTOR_FORWARD, MOTOR_BACKWARD);
Servo servo;
Controller controller;
uint8_t address[][6] = {"aaaaa", "aaaaa"};
uint8_t payload[13];
uint8_t speed = 0;
uint8_t lastSpeed = speed;
uint8_t direction = 0;
int16_t rotation = 90;
int16_t lastRotation = 0;
bool radioConnected = false;

void setup()
{
  pinMode(LED, OUTPUT);
  analogWrite(LED, 100);

  radio.begin();
  radio.setPALevel(RF24_PA_MAX);
  radio.setDataRate(RF24_250KBPS);
  radio.setPayloadSize(13);
  radio.setAutoAck(false);
  radio.setChannel(125);
  radio.openReadingPipe(0, address[0]);
  // radio.openReadingPipe(1, address[1]);
  radio.startListening();

  motor.initialize();
  servo.attach(SERVO);
  servo.write(rotation);
}

void loop() {
  uint8_t pipe;

  if (radio.available(&pipe)) {
    uint8_t bytes = radio.getPayloadSize();
    radio.read(&payload, bytes);
    controller.load_state_from_payload(payload);

    if (!radioConnected) {
      radioConnected = true;
      analogWrite(LED, 0);
    }

    speed = 0;

    rotation = map(controller.getYaw(), -32768, 32767, 180, 0);

    if (rotation != lastRotation) {
      servo.write(rotation);
    }

    if (controller.getL2() == 0 && controller.getR2() == 0) {
      motor.stop();
    } else {
      if (controller.getL2() != 0) {
        direction = MOTOR_DIRECTION_BACKWARD;
        speed = map(controller.getL2(), 0, 32768, 0, 255);
      }

      if (controller.getR2() != 0) {
        direction = MOTOR_DIRECTION_FORWARD;
        speed = map(controller.getR2(), 0, 32768, 0, 255);
      }

      motor.setSpeed(speed);
      motor.setDirection(direction);
    }
  }

  if (lastSpeed != speed) {
    analogWrite(LED, speed);
  }

  lastSpeed = speed;
  lastRotation = rotation;
}
