#include <controller.h>

Controller::Controller() {
  r1 = false;
  l1 = false;
  start = false;
  select = false;
  triangle = false;
  square = false;
  circle = false;
  cross = false;
  l2 = 0;
  r2 = 0;
  yaw = 0;
  throttle = 0;
  pitch = 0;
  roll = 0;
}

void Controller::load_state_from_payload(uint8_t payload[13]) {
  cross = (payload[0] & 0x01) != 0;
  circle = (payload[0] & 0x02) != 0;
  square = (payload[0] & 0x04) != 0;
  triangle = (payload[0] & 0x08) != 0;
  select = (payload[0] & 0x10) != 0;
  start = (payload[0] & 0x20) != 0;
  l1 = (payload[0] & 0x40) != 0;
  r1 = (payload[0] & 0x40) != 0;
  l2 = ((uint16_t)payload[1] << 8) | payload[2];
  r2 = ((uint16_t)payload[3] << 8) | payload[4];
  yaw = (payload[5] << 8) | payload[6];
  throttle = (payload[7] << 8) | payload[8];
  pitch = (payload[9] << 8) | payload[10];
  roll = (payload[11] << 8) | payload[12];
}

bool Controller::getR1() {
  return r1;
}

bool Controller::getL1() {
  return l1;
}

bool Controller::getStart() {
  return start;
}

bool Controller::getSelect() {
  return select;
}

bool Controller::getTriangle() {
  return triangle;
}

bool Controller::getSquare() {
  return square;
}

bool Controller::getCircle() {
  return circle;
}

bool Controller::getCross() {
  return cross;
}

uint16_t Controller::getL2() {
  return l2;
}

uint16_t Controller::getR2() {
  return r2;
}

int16_t Controller::getYaw() {
  return yaw;
}

int16_t Controller::getThrottle() {
  return throttle;
}

int16_t Controller::getPitch() {
  return pitch;
}
