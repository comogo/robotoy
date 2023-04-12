#include <Arduino.h>
#include <LiquidCrystal_I2C.h>
#include <EEPROM.h>
#include <Servo.h>
#include <radio.h>
#include <motor.h>
#include <controller.h>
#include <led.h>
#include <state.h>
#include <utils.h>

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
#define ROTATION_LIMIT 30
#define EEPROM_ROTAION_MIDDLE_ADDRESS 0

uint8_t address[6] = "aaaaa";
LiquidCrystal_I2C lcd(LCD_ADDRESS, LCD_COLS, LCD_ROWS);
Radio radio(RADIO_CE, RADIO_CSN, RADIO_CHANNEL, address);
Motor motor(MOTOR_STANDBY, MOTOR_PWM, MOTOR_FORWARD, MOTOR_BACKWARD);
Led led(LED_PIN);
Servo servo;
Controller controller;
State state;
uint8_t payload[13];
uint8_t speed = 0;
uint8_t lastSpeed = speed;
int rotationMiddle = 0;
int rotation = 0;
int lastRotation = 0;

bool allowDisplayNotConnected = true;
bool allowDisplayConnected = true;


void store_rotation_middle(int rotation)
{
  EEPROM.put(EEPROM_ROTAION_MIDDLE_ADDRESS, rotation);
}

int initialize_rotation_middle()
{
  int center;

  EEPROM.get(EEPROM_ROTAION_MIDDLE_ADDRESS, center);

  if (center == 0xFFFF)
  {
    center = 90;
    store_rotation_middle(center);
  }

  return center;
}

int handle_rotation(int16_t value, int lastValue, int center)
{
  // Map to int8_t
  int prepared_value = map(value, -32768, 32767, -128, 127);

  // apply a cubic bezier curve to make the rotation more smooth
  prepared_value = belzier(prepared_value, 3);

  // limit the rotation to 30 degrees from the center
  prepared_value = map(prepared_value, -128, 127, max(center - ROTATION_LIMIT, 60), min(center + ROTATION_LIMIT, 140));


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

void state_running()
{
  if (controller.isSelectReleased() && state.bounced())
  {
    state.setSetupState();
    lcd.clear();
    lcd.setCursor(0, 0);
    lcd.print("SETUP");
    return;
  }

  led.fastBlink();

  lastRotation = handle_rotation(controller.getYaw(), lastRotation, rotationMiddle);
  lastSpeed = handle_direction(controller.getR2(), controller.getL2(), lastSpeed);

  if (allowDisplayConnected)
  {
    lcd.setCursor(0, 0);
    lcd.print("C");
    lcd.setCursor(0, 1);
    lcd.print("R: ");
    allowDisplayConnected = false;
  }

  lcd.setCursor(3, 1);
  lcd.print(lastRotation);
  lcd.print("   ");
}

void state_setup() {
  if (controller.isSelectReleased() && state.bounced())
  {
    state.setRunningState();
    lcd.clear();
    allowDisplayConnected = true;
    return;
  }

  led.on();
  lastRotation = handle_rotation(controller.getYaw(), lastRotation, rotationMiddle);
  lcd.setCursor(3, 1);
  lcd.print(lastRotation);
  lcd.print("   ");

  if (controller.isStartReleased())
  {
    rotationMiddle = lastRotation;
    store_rotation_middle(rotationMiddle);
    lcd.setCursor(0, 1);
    lcd.print("Saved!");
  }
}

void setup()
{
  lcd.init();
  lcd.backlight();
  lcd.setCursor(0, 0);
  lcd.print("Initializing");
  delay(3000);
  led.initialize();
  led.on();
  rotationMiddle = initialize_rotation_middle();
  radio.initialize();
  motor.initialize();
  servo.attach(SERVO);
  delay(1000);
  led.off();
  lcd.clear();
  state.setRunningState();
}

void loop()
{
  if (radio.is_initialized() && radio.available())
  {
    allowDisplayNotConnected = true;
    radio.read(&payload);
    controller.load_state_from_payload(payload);

    if (state.isRunning())
    {
      state_running();
    } else if (state.isSetup())
    {
      state_setup();
    }
  }
  else
  {
    led.slowBlink();
    allowDisplayConnected = true;

    if (state.isRunning() && allowDisplayNotConnected)
    {
      lcd.setCursor(0, 0);
      lcd.print("?");
      allowDisplayNotConnected = false;
    }
  }
}
