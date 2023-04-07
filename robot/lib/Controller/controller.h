#ifndef CONTROLLER_H
#define CONTROLLER_H

#include <Arduino.h>

class Controller {
  private:
    bool m_r1;
    bool m_l1;
    bool m_start;
    bool m_select;
    bool m_triangle;
    bool m_square;
    bool m_circle;
    bool m_cross;
    uint16_t m_l2; // 0 to 32_768
    uint16_t m_r2; // 0 to 32_768
    int16_t m_yaw; // -32_768 to 32_768
    int16_t m_throttle; // -32_768 to 32_768
    int16_t m_pitch; // -32_768 to 32_768
    int16_t m_roll; // -32_768 to 32_768
    bool m_selectReleased;
    bool m_startReleased;
    bool m_triangleReleased;
    bool m_squareReleased;
    bool m_circleReleased;
    bool m_crossReleased;
    bool m_l1Released;
    bool m_r1Released;

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
    bool isSelectReleased();
    bool isStartReleased();
    bool isTriangleReleased();
    bool isSquareReleased();
    bool isCircleReleased();
    bool isCrossReleased();
    bool isL1Released();
    bool isR1Released();
};

#endif
