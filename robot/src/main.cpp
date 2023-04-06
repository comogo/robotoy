#include <Arduino.h>
#include <Wire.h>
#include <LiquidCrystal_I2C.h>
#include <Servo.h>
#include <radio.h>
#include <motor.h>
#include <controller.h>
#include <led.h>

/*
  PWM pins: 3, 5, 6, 9, 10, 11
*/

#define MOTOR_STANDBY 2
#define MOTOR_PWM 5 // PWM
#define MOTOR_FORWARD 4
#define MOTOR_BACKWARD 3
#define LED_PIN 6
#define RADIO_CE 7
#define RADIO_CSN 8
#define RADIO_CHANNEL 125
#define SERVO 9 // PWM

#define LCD_ADDRESS 0x27
#define LCD_COLS 16
#define LCD_ROWS 2

#define MAX_SPEED 250
#define MAX_ROTATION 180
#define MIN_ROTATION 0
#define CENTER_ROTATION 90

uint8_t address[6] = "aaaaa";
LiquidCrystal_I2C lcd(LCD_ADDRESS);
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
  lcd.begin(LCD_COLS, LCD_ROWS);
  lcd.clear();
  lcd.print("Initializing...");
  led.initialize();
  led.on();
  radio.initialize();
  motor.initialize();
  servo.attach(SERVO);
  servo.write(rotation);
  delay(1000);
  led.off();
}

int handle_rotation(int16_t value, int lastValue)
{
  int prepared_value = map(value, -32768, 32767, MAX_ROTATION, MIN_ROTATION);

  if (prepared_value != lastValue)
  {
    servo.write(prepared_value);
  }

  return prepared_value;
}

uint16_t handle_direction(uint16_t forward_speed, uint16_t backward_speed, uint16_t lastSpeed)
{
  uint16_t speed = 0;

  if (backward_speed != 0)
  {
    motor.setDirection(MOTOR_DIRECTION_BACKWARD);
    speed = map(backward_speed, 0, 32768, 0, MAX_SPEED);
  }

  if (forward_speed != 0)
  {
    motor.setDirection(MOTOR_DIRECTION_FORWARD);
    speed = map(forward_speed, 0, 32768, 0, MAX_SPEED);
  }

  if (speed != lastSpeed)
  {
    motor.setSpeed(speed);
  }

  return speed;
}

void loop()
{
  if (radio.available())
  {
    led.fastBlink();
    lcd.setCursor(0, 0);
    lcd.print("C");
    radio.read(&payload);
    controller.load_state_from_payload(payload);

    lastRotation = handle_rotation(controller.getYaw(), lastRotation);
    lastSpeed = handle_direction(controller.getR2(), controller.getL2(), lastSpeed);
    lcd.setCursor(0, 1);
    lcd.print("R: ");
    lcd.print(lastRotation);
  }
  else
  {
    led.slowBlink();
    lcd.setCursor(0, 0);
    lcd.print("?");
  }
}
