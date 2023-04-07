#include <controller.h>

Controller::Controller() {
  m_r1 = false;
  m_l1 = false;
  m_start = false;
  m_select = false;
  m_triangle = false;
  m_square = false;
  m_circle = false;
  m_cross = false;
  m_l2 = 0;
  m_r2 = 0;
  m_yaw = 0;
  m_throttle = 0;
  m_pitch = 0;
  m_roll = 0;
  m_selectReleased = false;
  m_startReleased = false;
  m_triangleReleased = false;
  m_squareReleased = false;
  m_circleReleased = false;
  m_crossReleased = false;
  m_l1Released = false;
  m_r1Released = false;
}

void Controller::load_state_from_payload(uint8_t payload[13]) {
  bool newCross = (payload[0] & 0x01) != 0;
  bool newCircle = (payload[0] & 0x02) != 0;
  bool newSquare = (payload[0] & 0x04) != 0;
  bool newTriangle = (payload[0] & 0x08) != 0;
  bool newSelect = (payload[0] & 0x10) != 0;
  bool newStart = (payload[0] & 0x20) != 0;
  bool newL1 = (payload[0] & 0x40) != 0;
  bool newR1 = (payload[0] & 0x80) != 0;

  m_selectReleased = m_select && !newSelect;
  m_startReleased = m_start && !newStart;
  m_triangleReleased = m_triangle && !newTriangle;
  m_squareReleased = m_square && !newSquare;
  m_circleReleased = m_circle && !newCircle;
  m_crossReleased = m_cross && !newCross;
  m_l1Released = m_l1 && !newL1;
  m_r1Released = m_r1 && !newR1;

  m_cross = newCross;
  m_circle = newCircle;
  m_square = newSquare;
  m_triangle = newTriangle;
  m_select = newSelect;
  m_start = newStart;
  m_l1 = newL1;
  m_r1 = newR1;
  m_l2 = ((uint16_t)payload[1] << 8) | payload[2];
  m_r2 = ((uint16_t)payload[3] << 8) | payload[4];
  m_yaw = (payload[5] << 8) | payload[6];
  m_throttle = (payload[7] << 8) | payload[8];
  m_pitch = (payload[9] << 8) | payload[10];
  m_roll = (payload[11] << 8) | payload[12];
}

bool Controller::getR1() {
  return m_r1;
}

bool Controller::getL1() {
  return m_l1;
}

bool Controller::getStart() {
  return m_start;
}

bool Controller::getSelect() {
  return m_select;
}

bool Controller::getTriangle() {
  return m_triangle;
}

bool Controller::getSquare() {
  return m_square;
}

bool Controller::getCircle() {
  return m_circle;
}

bool Controller::getCross() {
  return m_cross;
}

uint16_t Controller::getL2() {
  return m_l2;
}

uint16_t Controller::getR2() {
  return m_r2;
}

int16_t Controller::getYaw() {
  return m_yaw;
}

int16_t Controller::getThrottle() {
  return m_throttle;
}

int16_t Controller::getPitch() {
  return m_pitch;
}

bool Controller::isSelectReleased() {
  return m_selectReleased;
}

bool Controller::isStartReleased() {
  return m_startReleased;
}

bool Controller::isTriangleReleased() {
  return m_triangleReleased;
}

bool Controller::isSquareReleased() {
  return m_squareReleased;
}

bool Controller::isCircleReleased() {
  return m_circleReleased;
}

bool Controller::isCrossReleased() {
  return m_crossReleased;
}

bool Controller::isL1Released() {
  return m_l1Released;
}

bool Controller::isR1Released() {
  return m_r1Released;
}
