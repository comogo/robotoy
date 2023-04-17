#include <Arduino.h>
#include <EEPROM.h>
#include <Servo.h>
#include <radio.h>
#include <motor.h>
#include <controller.h>
#include <led.h>
#include <state.h>
#include <timer.h>
#include <voltimeter.h>
#include <lcd.h>
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
#define VOLTIMETER_PIN A0

#define MAX_SPEED 250
#define ROTATION_LIMIT 30
#define EEPROM_ROTAION_MIDDLE_ADDRESS 0

uint8_t address[6] = "aaaaa";
Lcd lcd(LCD_ADDRESS, LCD_COLS, LCD_ROWS);
Radio radio(RADIO_CE, RADIO_CSN, RADIO_CHANNEL, address);
Motor motor(MOTOR_STANDBY, MOTOR_PWM, MOTOR_FORWARD, MOTOR_BACKWARD);
Led led(LED_PIN);
Servo servo;
Controller controller;
State state;
Voltimeter voltimeter(VOLTIMETER_PIN);
uint8_t payload[13];
uint8_t speed = 0;
uint8_t lastSpeed = speed;
int rotationMiddle = 0;
int rotation = 0;
int lastRotation = 0;
int lastConnectionSpeedRate = 0;
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

void handle_voltimeter()
{
  voltimeter.read();

  if (voltimeter.isFresh() && !state.isSetup())
  {
    lcd.showVoltage(voltimeter.getVoltage());
  }
}

void state_running()
{
  if (controller.isSelectReleased() && state.bounced())
  {
    state.setSetupState();
    lcd.showState(state);
    return;
  }

  led.fastBlink();

  lastRotation = handle_rotation(controller.getYaw(), lastRotation, rotationMiddle);
  lastSpeed = handle_direction(controller.getR2(), controller.getL2(), lastSpeed);

  if (allowDisplayConnected)
  {
    lcd.showConnected();
    allowDisplayConnected = false;
  }

  int rate = radio.getConnectionSpeedRate();
  if (lastConnectionSpeedRate != rate)
  {
    lcd.showConnectionSpeed(rate);
    lastConnectionSpeedRate = rate;
  }
}

void state_setup()
{
  if (controller.isSelectReleased() && state.bounced())
  {
    state.setRunningState();
    lcd.clear();
    allowDisplayConnected = true;
    return;
  }

  led.on();
  lastRotation = handle_rotation(controller.getYaw(), lastRotation, rotationMiddle);
  lcd.showRotation(lastRotation);

  if (controller.isStartReleased())
  {
    rotationMiddle = lastRotation;
    store_rotation_middle(rotationMiddle);
    lcd.showSaved();
  }
}

void state_disconnected()
{
  led.slowBlink();
  allowDisplayConnected = true;

  lastRotation = handle_rotation(rotationMiddle, lastRotation, rotationMiddle);
  lastSpeed = handle_direction(0, 0, lastSpeed);

  if (allowDisplayNotConnected)
  {
    lcd.showDisconnected();
    allowDisplayNotConnected = false;
  }
}

void execute_state()
{
  if (state.isRunning())
  {
    state_running();
  }
  else if (state.isSetup())
  {
    state_setup();
  }
  else if (state.isDisconnected())
  {
    state_disconnected();
  }
}

void set_state_from_connection(bool connected)
{
  if (connected)
  {
    allowDisplayNotConnected = true;
    if (state.isDisconnected())
    {
      state.setRunningState();
    }
  }
  else
  {
    state.setDisconnectedState();
  }
}

void setup()
{
  lcd.initialize();
  lcd.showState(state);
  voltimeter.initialize();
  led.initialize();
  led.on();
  rotationMiddle = initialize_rotation_middle();
  radio.initialize();
  motor.initialize();
  servo.attach(SERVO);
  delay(1000);
  led.off();
  lcd.clear();
  state.setDisconnectedState();
}

void loop()
{
  if (radio.isInitialized() && radio.available())
  {
    radio.read(&payload);
    controller.load_state_from_payload(payload);
  }

  set_state_from_connection(radio.isConnected());
  execute_state();
  handle_voltimeter();
}
