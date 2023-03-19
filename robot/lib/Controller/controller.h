#ifndef CONTROLLER_H
#define CONTROLLER_H

#include <Arduino.h>

class Controller {
  private:
    bool r1;
    bool l1;
    bool start;
    bool select;
    bool triangle;
    bool square;
    bool circle;
    bool cross;
    uint16_t l2; // 0 to 32_768
    uint16_t r2; // 0 to 32_768
    int16_t yaw; // -32_768 to 32_768
    int16_t throttle; // -32_768 to 32_768
    int16_t pitch; // -32_768 to 32_768
    int16_t roll; // -32_768 to 32_768

  public:
    Controller();
    void load_state_from_payload(uint8_t payload[13]);
    bool getR1();
    bool getL1();
    bool getStart();
    bool getSelect();
    bool getTriangle();
    bool getSquare();
    bool getCircle();
    bool getCross();
    uint16_t getL2();
    uint16_t getR2();
    int16_t getYaw();
    int16_t getThrottle();
    int16_t getPitch();
    int16_t getRoll();
};

#endif
