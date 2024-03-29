#include <lcd.h>
#include <utils.h>

Lcd::Lcd(uint8_t address, uint8_t cols, uint8_t rows)
{
  m_lcd = new LiquidCrystal_I2C(address, cols, rows);
  m_initialized = false;
}

void Lcd::initialize()
{
  if (m_initialized)
  {
    return;
  }

  m_lcd->init();
  m_lcd->backlight();
  m_lcd->setCursor(0, 0);
  m_initialized = true;
}

void Lcd::showDisconnected()
{
  m_lcd->setCursor(0, 0);
  m_lcd->print("?               ");
}

void Lcd::showConnected()
{
  m_lcd->setCursor(0, 0);
  m_lcd->print("C ");
}

void Lcd::showPowerStats(float voltage, float current)
{
  m_lcd->setCursor(0, 1);
  m_lcd->print(voltage);
  m_lcd->print("V ");

  if (current < 1)
  {
    int currentMilli = current * 1000;
    m_lcd->print(currentMilli);
    m_lcd->print("mA");
  } else {
    m_lcd->print(current);
    m_lcd->print("A");
  }
}

void Lcd::showConnectionSpeed(int connectionSpeedRate)
{
  char rate[5];
  m_lcd->setCursor(2, 0);
  toStringWithPadding(rate, connectionSpeedRate, 4, ' ');
  m_lcd->print(rate);
  m_lcd->print("p/s");
}

void Lcd::showState(State state)
{
  if (state.isInitializing())
  {
    m_lcd->clear();
    m_lcd->setCursor(0, 0);
    m_lcd->print("INITIALIZING");
    return;
  }

  if (state.isSetup())
  {
    m_lcd->clear();
    m_lcd->setCursor(0, 0);
    m_lcd->print("SETUP");
    return;
  }
}

void Lcd::clear()
{
  m_lcd->clear();
}

void Lcd::showRotation(int16_t rotation)
{
  m_lcd->setCursor(3, 1);
  m_lcd->print(rotation);
  m_lcd->print("   ");
}

void Lcd::showSaved()
{
  m_lcd->setCursor(0, 1);
  m_lcd->print("Saved!");
}
