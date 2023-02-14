extern crate embedded-nrf24l01;
extern crate rppal;

use embedded_nrf24l01::NRF24L01;
use rppal::{spi, spi::{Spi}, gpio, gpio::{Gpio}};

pub struct Radio {
    device: NRF24L01,
}

impl Radio {
    pub fn new(ce_pin: u8, csn_pin: u8) -> Self {
        let gpio: Gpio = Gpio::new().unwrap();
        let mut ce: gpio::OutputPin = gpio.get(ce_pin).unwrap().into_output();
        let mut csn: gpio::OutputPin = gpio.get(csn_pin).unwrap().into_output();
        let spi: Spi = Spi::new(
            spi::Bus::Spi0,
            spi::SlaveSelect::Ss0,
            1_000_000,
            spi::Mode::Mode0,
        ).unwrap();

        let device = NRF24L01::new(ce, csn, spi).unwrap();

        Radio {
            device
        }
    }
}
