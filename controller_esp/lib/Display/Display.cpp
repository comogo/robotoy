#include <Display.h>

Display::Display()
{
  mDisplay = new Adafruit_SSD1306(128, 64, &Wire, RST_OLED);
  mOrientation = DISPLAY_ORIENTATION_NORMAL;
}

Display::~Display()
{
  delete mDisplay;
}

void Display::begin()
{
  mDisplay->begin(SSD1306_SWITCHCAPVCC, 0x3C);
  mDisplay->setTextSize(1);
  mDisplay->setTextColor(WHITE);
  mDisplay->setCursor(0, 0);
  mDisplay->clearDisplay();
  mDisplay->display();
}

void Display::clear()
{
  mDisplay->clearDisplay();
}

void Display::drawString(int x, int y, String text)
{
  mDisplay->setCursor(x, y);
  mDisplay->print(text);
}

void Display::display()
{
  mDisplay->display();
}

void Display::flip()
{
  if (mOrientation == DISPLAY_ORIENTATION_NORMAL)
  {
    mDisplay->setRotation(DISPLAY_ORIENTATION_FLIPPED);
  } else {
    mDisplay->setRotation(DISPLAY_ORIENTATION_NORMAL);
  }
}
