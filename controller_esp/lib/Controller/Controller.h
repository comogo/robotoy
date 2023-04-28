#ifndef CONTROLLER_H
#define CONTROLLER_H

#include <Arduino.h>

#define CONTROLLER_LEFT_STICK_Y GPIO_NUM_37
#define CONTROLLER_LEFT_STICK_X GPIO_NUM_38
#define CONTROLLER_RIGHT_STICK_Y GPIO_NUM_39
#define CONTROLLER_RIGHT_STICK_X GPIO_NUM_2
#define CONTROLLER_BUTTON_A GPIO_NUM_12
#define CONTROLLER_BUTTON_B GPIO_NUM_13
#define CONTROLLER_BUTTON_C GPIO_NUM_23
#define CONTROLLER_BUTTON_D GPIO_NUM_17
#define CONTROLLER_CALIBRATION_CYCLES 10

class Controller {
  private:
    int8_t mLeftStickY;
    int8_t mLeftStickX;
    int8_t mRightStickY;
    int8_t mRightStickX;
    int8_t mLeftStickYCalibration;
    int8_t mLeftStickXCalibration;
    int8_t mRightStickYCalibration;
    int8_t mRightStickXCalibration;
    bool mButtonA;
    bool mButtonB;
    bool mButtonC;
    bool mButtonD;
    bool mInvertAxis;

    int8_t normalizeAnalogValue(u_int16_t value, int8_t calibrationValue);

  public:
    Controller(bool invertAxis);
    ~Controller();
    void calibrate();
    void read();
    int8_t getLeftStickY();
    int8_t getLeftStickX();
    int8_t getRightStickY();
    int8_t getRightStickX();
    bool getButtonA();
    bool getButtonB();
    bool getButtonC();
    bool getButtonD();
};

#endif
