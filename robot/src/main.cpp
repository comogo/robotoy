#include <Arduino.h>
#include <SPI.h>
#include "RF24.h"
#include "printf.h"

#define MOTOR_PWM 6
#define MOTOR_FORWARD 4
#define MOTOR_BACKWARD 5
#define LED 3
// #define DEBUG

struct Controller {
  bool r1;
  bool l1;
  bool start;
  bool select;
  bool triangle;
  bool square;
  bool circle;
  bool x;
  uint16_t l2; // 0 to 32_768
  uint16_t r2; // 0 to 32_768
  int16_t yaw;
  int16_t throttle;
  int16_t pitch;
  int16_t roll;
};

RF24 radio(7, 8);  // using pin 7 for the CE pin, and pin 8 for the CSN pin
uint8_t address[][6] = { "aaaaa", "aaaaa" };
Controller controller;
uint8_t payload[13];
uint8_t speed = 0;
uint8_t direction = 1; // 1 = FORWARD; 2 = BACKWARD

void setupMotor() {
  pinMode(MOTOR_FORWARD, OUTPUT);
  pinMode(MOTOR_BACKWARD, OUTPUT);
  pinMode(MOTOR_PWM, OUTPUT);
  digitalWrite(MOTOR_FORWARD, LOW);
  digitalWrite(MOTOR_BACKWARD, LOW);
  analogWrite(MOTOR_PWM, 0);
}

void setMotorSpeed(int velocity) {
  analogWrite(MOTOR_PWM, velocity);
}

// 0 = BREAK
// 1 = FORWARD
// 2 = BACKWARD
void setMotorDirection(int direction) {
  switch (direction) {
    case 0: // break
      digitalWrite(MOTOR_FORWARD, HIGH);
      digitalWrite(MOTOR_BACKWARD, HIGH);
      break;
    case 1: // forward
      digitalWrite(MOTOR_FORWARD, HIGH);
      digitalWrite(MOTOR_BACKWARD, LOW);
      break;
    case 2: // backward
      digitalWrite(MOTOR_FORWARD, LOW);
      digitalWrite(MOTOR_BACKWARD, HIGH);
      break;
    default:
      digitalWrite(MOTOR_FORWARD, LOW);
      digitalWrite(MOTOR_BACKWARD, LOW);
    }
}

void setup()
{
#ifdef DEBUG
  Serial.begin(115200);
  while (!Serial) {
    // some boards need to wait to ensure access to serial over USB
  }
  printf_begin();
#endif

  radio.begin();
  radio.setPALevel(RF24_PA_MAX);
  radio.setDataRate(RF24_250KBPS);
  radio.setPayloadSize(13);
  radio.setAutoAck(false);
  radio.setChannel(125);
  radio.openReadingPipe(0, address[0]);
  // radio.openReadingPipe(1, address[1]);
  radio.startListening();

  pinMode(LED, OUTPUT);
  analogWrite(LED, 0);
  setupMotor();

#ifdef DEBUG
  radio.printDetails();
#endif
}

void map_payload(uint8_t p[13], Controller *c) {
  c->x = (p[0] & 0x01) != 0;
  c->circle = (p[0] & 0x02) != 0;
  c->square = (p[0] & 0x04) != 0;
  c->triangle = (p[0] & 0x08) != 0;
  c->select = (p[0] & 0x10) != 0;
  c->start = (p[0] & 0x20) != 0;
  c->l1 = (p[0] & 0x40) != 0;
  c->r1 = (p[0] & 0x40) != 0;
  c->l2 = ((uint16_t)p[1] << 8) | p[2];
  c->r2 = ((uint16_t)p[3] << 8) | p[4];
  c->yaw = (p[5] << 8) | p[6];
  c->throttle = (p[7] << 8) | p[8];
  c->pitch = (p[9] << 8) | p[10];
  c->roll = (p[11] << 8) | p[12];
}

void loop() {
  uint8_t pipe;
  if (radio.available(&pipe)) {
    uint8_t bytes = radio.getPayloadSize();
    radio.read(&payload, bytes);
    map_payload(payload, &controller);

    speed = 0;
    direction = 0;

    if (controller.r2 != 0) {
      direction = 1;
      speed = map(controller.r2, 0, 32768, 0, 255);
    }

    if (controller.l2 != 0) {
      direction = 2;
      speed = map(controller.l2, 0, 32768, 0, 255);
    }

#ifdef DEBUG
  Serial.print("SPEED: ");
  Serial.print(speed);
  Serial.print("DIR: ");
  Serial.print(direction);
#endif
  analogWrite(LED, speed);
  setMotorSpeed(speed);
  setMotorDirection(direction);
  }
}

