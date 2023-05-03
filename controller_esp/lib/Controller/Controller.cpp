#include <Controller.h>

Controller::Controller(bool invertAxis = false) {
  mLeftStickY = 0;
  mLeftStickX = 0;
  mRightStickY = 0;
  mRightStickX = 0;
  mButtonA = false;
  mButtonB = false;
  mButtonC = false;
  mButtonD = false;
  mLeftStickYCalibration = 0;
  mLeftStickXCalibration = 0;
  mRightStickYCalibration = 0;
  mRightStickXCalibration = 0;
  mInvertAxis = invertAxis;

  pinMode(CONTROLLER_LEFT_STICK_Y, INPUT);
  pinMode(CONTROLLER_LEFT_STICK_X, INPUT);
  pinMode(CONTROLLER_RIGHT_STICK_Y, INPUT);
  pinMode(CONTROLLER_RIGHT_STICK_X, INPUT);
  pinMode(CONTROLLER_BUTTON_A, INPUT_PULLUP);
  pinMode(CONTROLLER_BUTTON_B, INPUT_PULLUP);
  // pinMode(CONTROLLER_BUTTON_C, INPUT_PULLUP);
  // pinMode(CONTROLLER_BUTTON_D, INPUT_PULLUP);
  analogSetPinAttenuation(CONTROLLER_LEFT_STICK_Y, ADC_11db);
  analogSetPinAttenuation(CONTROLLER_LEFT_STICK_X, ADC_11db);
  analogSetPinAttenuation(CONTROLLER_RIGHT_STICK_Y, ADC_11db);
  analogSetPinAttenuation(CONTROLLER_RIGHT_STICK_X, ADC_11db);
}

Controller::~Controller() {
}

void Controller::calibrate() {
  u_int16_t leftStickYCalibrationAvg = 0;
  u_int16_t leftStickXCalibrationAvg = 0;
  u_int16_t rightStickYCalibrationAvg = 0;
  u_int16_t rightStickXCalibrationAvg = 0;

  for (int i = 0; i < CONTROLLER_CALIBRATION_CYCLES; i++) {
    leftStickYCalibrationAvg  += analogRead(CONTROLLER_LEFT_STICK_Y);
    leftStickXCalibrationAvg  += analogRead(CONTROLLER_LEFT_STICK_X);
    rightStickYCalibrationAvg += analogRead(CONTROLLER_RIGHT_STICK_Y);
    rightStickXCalibrationAvg += analogRead(CONTROLLER_RIGHT_STICK_X);
    delay(30);
  }

  leftStickYCalibrationAvg = leftStickYCalibrationAvg / CONTROLLER_CALIBRATION_CYCLES;
  leftStickXCalibrationAvg = leftStickXCalibrationAvg / CONTROLLER_CALIBRATION_CYCLES;
  rightStickYCalibrationAvg = rightStickYCalibrationAvg / CONTROLLER_CALIBRATION_CYCLES;
  rightStickXCalibrationAvg = rightStickXCalibrationAvg / CONTROLLER_CALIBRATION_CYCLES;

  mLeftStickYCalibration  = map(leftStickYCalibrationAvg, 0, 4096, -128, 127);
  mLeftStickXCalibration  = map(leftStickXCalibrationAvg, 0, 4096, -128, 127);
  mRightStickYCalibration = map(rightStickYCalibrationAvg, 0, 4096, -128, 127);
  mRightStickXCalibration = map(rightStickXCalibrationAvg, 0, 4096, -128, 127);
}

int8_t Controller::normalizeAnalogValue(u_int16_t value, int8_t calibrationValue) {
  int8_t mappedValue = map(value, 0, 4095, -128, 127);
  int16_t normalizedValue = mappedValue + (calibrationValue * -1);

  // Deadzone
  if (normalizedValue >= -5 && normalizedValue <= 5) {
    return 0;
  }

  normalizedValue -= (calibrationValue * -1);

  if (mInvertAxis) {
    normalizedValue *= -1;
  }

  if (normalizedValue > 125) { // 125 because of the deadzone
    normalizedValue = 127;
  }

  if (normalizedValue < -125) { // -125 because of the deadzone
    normalizedValue = -128;
  }

  return normalizedValue;
}

void Controller::read() {
  mLeftStickY = normalizeAnalogValue(analogRead(CONTROLLER_LEFT_STICK_Y), mLeftStickYCalibration);
  mLeftStickX = normalizeAnalogValue(analogRead(CONTROLLER_LEFT_STICK_X), mLeftStickXCalibration);
  mRightStickY = normalizeAnalogValue(analogRead(CONTROLLER_RIGHT_STICK_Y), mRightStickYCalibration);
  mRightStickX = normalizeAnalogValue(analogRead(CONTROLLER_RIGHT_STICK_X), mRightStickXCalibration);
  mButtonA = digitalRead(CONTROLLER_BUTTON_A) == LOW;
  mButtonB = digitalRead(CONTROLLER_BUTTON_B) == LOW;
  // mButtonC = digitalRead(CONTROLLER_BUTTON_C) == LOW;
  // mButtonD = digitalRead(CONTROLLER_BUTTON_D) == LOW;
}

int8_t Controller::getLeftStickY() {
  return mLeftStickY;
}

int8_t Controller::getLeftStickX() {
  return mLeftStickX;
}

int8_t Controller::getRightStickY() {
  return mRightStickY;
}

int8_t Controller::getRightStickX() {
  return mRightStickX;
}

bool Controller::getButtonA() {
  return mButtonA;
}

bool Controller::getButtonB() {
  return mButtonB;
}

bool Controller::getButtonC() {
  return mButtonC;
}

bool Controller::getButtonD() {
  return mButtonD;
}
