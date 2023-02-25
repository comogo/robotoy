extern crate rppal;

use rppal::{
    gpio,
    gpio::{Gpio, OutputPin},
    spi,
    spi::Segment,
    spi::Spi,
};
use std::thread::sleep;
use std::time::Duration;

type Command = u8;
type Register = u8;

const SPI_SPEED: u32 = 10_000_000;

pub fn radio(ce: u8) {
    let spi: Spi = Spi::new(
        spi::Bus::Spi0,
        spi::SlaveSelect::Ss0,
        SPI_SPEED,
        spi::Mode::Mode0,
    )
    .unwrap();

    sleep(Duration::from_millis(5));

    let mut data: [u8; 2] = [0, 0];
    spi.transfer(&mut data, &[0u8, 0u8]).unwrap();
    println!("CONFIG: {:#010b} {:#010b}", data[0], data[1]);

    spi.transfer(&mut data, &[3u8, 0u8]).unwrap();
    println!("SETUP_AW: {:#010b} {:#010b}", data[0], data[1]);
}
