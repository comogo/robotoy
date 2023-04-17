#ifndef LCD_H
#define LCD_H

#include <Arduino.h>
#include <LiquidCrystal_I2C.h>
#include <state.h>

#define LCD_ADDRESS 0x27
#define LCD_COLS 16
#define LCD_ROWS 2

class Lcd
{
private:
  LiquidCrystal_I2C *m_lcd;
  bool m_initialized;

public:
  Lcd(uint8_t address, uint8_t cols, uint8_t rows);
  void initialize();
  void showDisconnected();
  void showConnected();
  void showRotation(int16_t rotation);
  void showPowerStats(float voltage, float current);
  void showConnectionSpeed(int connectionSpeed);
  void showState(State state);
  void showSaved();
  void clear();
};

#endif
