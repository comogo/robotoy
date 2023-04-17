#ifndef VOLTIMETER_H
#define VOLTIMETER_H

#include <timer.h>

#define VOLTIMETER_READ_SAMPLES 10
#define VOLTIMETER_INTERVAL 1000
#define VOLTIMETER_R1 3.3
#define VOLTIMETER_R2 4.7

class Voltimeter
{
  private:
    int m_vpin;
    int m_cpin;
    float m_voltage;
    float m_current;
    bool m_fresh;
    Timer *m_timer;

  public:
    Voltimeter(int vAnalogPin, int cAnalogPin);
    ~Voltimeter();
    void initialize();

    /*
      Reads the voltage and current from the analog pins.
      It only reads the values if the timer has expired.

      tip for analogRead:
        5V is the max voltage of the Arduino so,
        1024 is the max value of analogRead
        resolution is 5V / 1024 = 0.004882813V

        Vin = (Vout * (R1 + R2)) / R2

      tip for current calculation:
        ACS712 30A sensor has a sensitivity of 66mV/A
        the output voltage is 2.5V when the current is 0A since the center is:
        VCC * 0.5 = 5V * 0.5 = 2.5V

        so, the current is:
        milliV = ((Vout * 0.004882813) - 2.5V) / 1000
        current = milliV / 66
    */
    void read();

    /*
      Returns the voltage in volts
    */
    float getVoltage();

    /*
      Returns the current in amps
    */
    float getCurrent();

    /*
      Returns true if the voltage and current are fresh readings
    */
    bool isFresh();
};

#endif
