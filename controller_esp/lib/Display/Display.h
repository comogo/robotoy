#ifndef DISPLAY_H
#define DISPLAY_H

#include <Arduino.h>
#include <Wire.h>
#include <Adafruit_SSD1306.h>

#define DISPLAY_ORIENTATION_NORMAL 1
#define DISPLAY_ORIENTATION_FLIPPED 4

class Display
{
  private:
    Adafruit_SSD1306 *mDisplay;
    uint8_t mOrientation;

  public:
    Display();
    ~Display();
    void begin();
    void clear();
    void drawString(int x, int y, String text);
    void display();
    void flip();
};

#endif
