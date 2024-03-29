extern crate rppal;

use rppal::{gpio, spi, spi::Spi};
use std::fmt;
use std::thread::sleep;
use std::time::Duration;

const SPI_SPEED: u32 = 10_000_000;

// CONFIG register bits
const MASK_RX_DR: u8 = 0x40;
const MASK_TX_DS: u8 = 0x20;
const MASK_MAX_RT: u8 = 0x10;
const EN_CRC: u8 = 0x08;
const CRCO: u8 = 0x04;
const PWR_UP: u8 = 0x02;
const PRIM_RX: u8 = 0x01;

// EN_AA register bits
const ENAA_P5: u8 = 0x20;
const ENAA_P4: u8 = 0x10;
const ENAA_P3: u8 = 0x08;
const ENAA_P2: u8 = 0x04;
const ENAA_P1: u8 = 0x02;
const ENAA_P0: u8 = 0x01;

// EN_RXADDR register bits
const ERX_P5: u8 = 0x20;
const ERX_P4: u8 = 0x10;
const ERX_P3: u8 = 0x08;
const ERX_P2: u8 = 0x04;
const ERX_P1: u8 = 0x02;
const ERX_P0: u8 = 0x01;

// SETUP_AW register bits
const AW: u8 = 0x03;

// SETUP_RETR register bits
const ARD: u8 = 0xF0;
const ARC: u8 = 0x0F;

// RF_CH register bits
const RF_CH: u8 = 0x7F;

// RF_SETUP register bits
const CONT_WAVE: u8 = 0x80;
const RF_DR_LOW: u8 = 0x20;
const PLL_LOCK: u8 = 0x10;
const RF_DR_HIGH: u8 = 0x08;
const RF_PWR: u8 = 0x06;

// STATUS register bits
const RX_DR: u8 = 0x40;
const TX_DS: u8 = 0x20;
const MAX_RT: u8 = 0x10;
const RX_P_NO: u8 = 0x0E;
const TX_FULL: u8 = 0x01;

// OBSERVE_TX register bits
const PLOS_CNT: u8 = 0xF0;
const ARC_CNT: u8 = 0x0F;

// RPD register bits
const RPD: u8 = 0x01;

// RX_PW_PX register bits
const RX_PW_P0: u8 = 0x3F;
const RX_PW_P1: u8 = 0x3F;
const RX_PW_P2: u8 = 0x3F;
const RX_PW_P3: u8 = 0x3F;
const RX_PW_P4: u8 = 0x3F;
const RX_PW_P5: u8 = 0x3F;

// FIFO_STATUS register bits
const TX_REUSE: u8 = 0x40;
const FIFO_FULL: u8 = 0x20;
const TX_EMPTY: u8 = 0x10;
const RX_FULL: u8 = 0x02;
const RX_EMPTY: u8 = 0x01;

// DYNPD register bits
const DPL_P5: u8 = 0x20;
const DPL_P4: u8 = 0x10;
const DPL_P3: u8 = 0x08;
const DPL_P2: u8 = 0x04;
const DPL_P1: u8 = 0x02;
const DPL_P0: u8 = 0x01;

// FEATURE register bits
const EN_DPL: u8 = 0x04;
const EN_ACK_PAY: u8 = 0x02;
const EN_DYN_ACK: u8 = 0x01;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum DataRate {
    _250Kbps = 0x20,
    _1Mbps = 0x00,
    _2Mbps = 0x08,
}

impl fmt::Display for DataRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DataRate::_250Kbps => write!(f, "250Kbps"),
            DataRate::_1Mbps => write!(f, "1Mbps"),
            DataRate::_2Mbps => write!(f, "2Mbps"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum PowerLevel {
    _18dBm = 0x00,
    _12dBm = 0x02,
    _6dBm = 0x04,
    _0dBm = 0x06,
}

impl fmt::Display for PowerLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PowerLevel::_18dBm => write!(f, "-18dBm"),
            PowerLevel::_12dBm => write!(f, "-12dBm"),
            PowerLevel::_6dBm => write!(f, "-6dBm"),
            PowerLevel::_0dBm => write!(f, "0dBm"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Command {
    R_REGISTER = 0x00,
    W_REGISTER = 0x20,
    R_RX_PAYLOAD = 0x61,
    W_TX_PAYLOAD = 0xA0,
    FLUSH_TX = 0xE1,
    FLUSH_RX = 0xE2,
    REUSE_TX_PL = 0xE3,
    R_RX_PL_WID = 0x60,
    W_ACK_PAYLOAD = 0xA8,
    W_TX_PAYLOAD_NOACK = 0xB0,
    NOP = 0xFF,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Register {
    CONFIG = 0x00,
    EN_AA,
    EN_RXADDR,
    SETUP_AW,
    SETUP_RETR,
    RF_CH,
    RF_SETUP,
    STATUS,
    OBSERVE_TX,
    RPD,
    RX_ADDR_P0,
    RX_ADDR_P1,
    RX_ADDR_P2,
    RX_ADDR_P3,
    RX_ADDR_P4,
    RX_ADDR_P5,
    TX_ADDR,
    RX_PW_P0,
    RX_PW_P1,
    RX_PW_P2,
    RX_PW_P3,
    RX_PW_P4,
    RX_PW_P5,
    FIFO_STATUS,
    DYNPD = 0x1C,
    FEATURE,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum RF24Error {
    GpioError(gpio::Error),
    SpiError(spi::Error),
    InvalidChannel,
    InvalidAddressWidth,
}

#[derive(Debug)]
pub struct RF24 {
    spi: Spi,
    ce_pin: gpio::OutputPin,
}

impl RF24 {
    pub fn new(ce_pin_number: u8) -> Result<RF24, RF24Error> {
        let ce_pin = gpio::Gpio::new()
            .map_err(|e| RF24Error::GpioError(e))?
            .get(ce_pin_number)
            .map_err(|e| RF24Error::GpioError(e))?
            .into_output_low();

        let spi: Spi = Spi::new(
            spi::Bus::Spi0,
            spi::SlaveSelect::Ss0,
            SPI_SPEED,
            spi::Mode::Mode0,
        )
        .map_err(|e| RF24Error::SpiError(e))?;

        sleep(Duration::from_millis(5));

        Ok(RF24 { spi, ce_pin })
    }

    /// Send a command to the NRF24L01+ module and reads the response.
    ///
    /// It returns the number of bytes read.
    fn command(&self, data_out: &[u8], data_in: &mut [u8]) -> Result<usize, RF24Error> {
        self.spi
            .transfer(data_in, data_out)
            .map_err(|e| RF24Error::SpiError(e))
    }

    /// Reads a register from the NRF24L01+ module.
    ///
    /// The `size` parameter is the number of bytes to read.
    /// The `data` parameter is the buffer where the read data will be stored.
    fn read_register(&self, reg: Register) -> Result<u8, RF24Error> {
        let mut data_in = [0u8; 2];
        let reg: u8 = Command::R_REGISTER as u8 | reg as u8;
        self.command(&[reg, 0], &mut data_in)?;
        Ok(data_in[1])
    }

    fn read_address(&self, reg: Register, size: usize, data: &mut [u8]) -> Result<(), RF24Error> {
        let mut data_out: Vec<u8> = vec![0; size + 1];
        let mut data_in: Vec<u8> = vec![0; size + 1];
        data_out[0] = Command::R_REGISTER as u8 | reg as u8;
        self.command(&data_out, &mut data_in)?;
        data.copy_from_slice(&data_in[1..]);
        Ok(())
    }

    /// Writes a register to the NRF24L01+ module.
    ///
    /// The `value` parameter is the data to write.
    fn write_register(&self, reg: Register, value: u8) -> Result<(), RF24Error> {
        let mut data_in = [0u8; 2];
        let register = Command::W_REGISTER as u8 | reg as u8;
        self.command(&[register, value], &mut data_in)?;
        Ok(())
    }

    /// Writes an full address, usualy 5 bytes to the NRF24L01+ module.
    fn write_full_address(&self, reg: Register, address: &[u8]) -> Result<(), RF24Error> {
        let size = address.len();
        let mut data_out: Vec<u8> = vec![0; size + 1];
        let mut data_in: Vec<u8> = vec![0; size + 1];
        data_out[0] = Command::W_REGISTER as u8 | reg as u8;
        data_out[1..].copy_from_slice(address);
        self.command(&data_out, &mut data_in)?;
        Ok(())
    }

    /// Flushes the TX FIFO.
    fn flush_tx(&self) -> Result<(), RF24Error> {
        let mut data_in = [0u8; 2];
        self.command(&[Command::FLUSH_TX as u8, 0], &mut data_in)?;
        Ok(())
    }

    /// Flushes the RX FIFO.
    fn flush_rx(&self) -> Result<(), RF24Error> {
        let mut data_in = [0u8; 2];
        self.command(&[Command::FLUSH_RX as u8, 0], &mut data_in)?;
        Ok(())
    }

    /// Writes a payload to the TX FIFO.
    fn write_payload(&self, data: &[u8]) -> Result<(), RF24Error> {
        let mut data_out: Vec<u8> = vec![0; data.len() + 1];
        let mut data_in: Vec<u8> = vec![0; data.len() + 1];
        data_out[0] = Command::W_TX_PAYLOAD as u8;
        data_out[1..].copy_from_slice(data);
        self.command(&data_out, &mut data_in)?;
        Ok(())
    }

    /// Enable CE pin to start transmission.
    fn set_ce_high(&mut self) {
        self.ce_pin.set_high();
    }

    /// Disable CE pin to stop transmission.
    fn set_ce_low(&mut self) {
        self.ce_pin.set_low();
    }

    fn print_details(&self) -> Result<(), RF24Error> {
        let mut register = self.read_register(Register::STATUS)?;
        print!("STATUS\t\t= {:#04x}", register);
        print!(" RX_DR={}", (register & RX_DR) >> 6);
        print!(" TX_DS={}", (register & TX_DS) >> 5);
        print!(" MAX_RT={}", (register & MAX_RT) >> 4);
        print!(" RX_P_NO={}", (register & RX_P_NO) >> 1);
        println!(" TX_FULL={}", register & TX_FULL);

        let mut full_address = [0u8; 5];
        self.read_address(Register::RX_ADDR_P0, 5, &mut full_address)?;
        print!("RX_ADDR_P0-1    = 0x");
        for i in full_address {
            print!("{:02x}", i);
        }

        self.read_address(Register::RX_ADDR_P1, 5, &mut full_address)?;
        print!(" 0x");
        for i in full_address {
            print!("{:02x}", i);
        }

        let mut short_address: u8 = 0;
        print!("\nRX_ADDR_P2-5   = ");
        short_address = self.read_register(Register::RX_ADDR_P2)?;
        print!(" 0x{:02x}", short_address);
        short_address = self.read_register(Register::RX_ADDR_P3)?;
        print!(" 0x{:02x}", short_address);
        short_address = self.read_register(Register::RX_ADDR_P4)?;
        print!(" 0x{:02x}", short_address);
        short_address = self.read_register(Register::RX_ADDR_P5)?;
        println!(" 0x{:02x}", short_address);

        self.read_address(Register::TX_ADDR, 5, &mut full_address)?;
        print!("TX_ADDR         = 0x");
        for i in full_address {
            print!("{:02x}", i);
        }
        println!("");

        register = self.read_register(Register::EN_AA)?;
        println!("EN_AA           = {:#04x}", register);

        register = self.read_register(Register::EN_RXADDR)?;
        println!("EN_RXADDR       = {:#04x}", register);

        register = self.read_register(Register::RF_CH)?;
        println!("RF_CH           = {:#04x}", register);

        register = self.read_register(Register::RF_SETUP)?;
        println!("RF_SETUP        = {:#04x}", register);

        register = self.read_register(Register::CONFIG)?;
        println!("CONFIG          = {:#04x}", register);

        register = self.read_register(Register::DYNPD)?;
        let feature = self.read_register(Register::FEATURE)?;
        println!("DYNPD/FEATURE   = {:#04x} {:#04x}", register, feature);

        Ok(())
    }
}

pub struct Radio {
    rf24: RF24,
    address: [u8; 5],
    rate: DataRate,
    power_level: PowerLevel,
    channel: u8,
    ce_pin_number: u8,
}

impl Radio {
    pub fn new(
        address: &str,
        rate: DataRate,
        power_level: PowerLevel,
        channel: u8,
        ce_pin_number: u8,
    ) -> Result<Radio, RF24Error> {
        let rf24 = RF24::new(ce_pin_number)?;

        let mut radio = Radio {
            rf24,
            rate,
            power_level,
            address: [0; 5],
            channel: 0,
            ce_pin_number,
        };

        radio.set_address(address)?;
        radio.set_channel(channel)?;

        Ok(radio)
    }

    pub fn set_address(&mut self, address: &str) -> Result<(), RF24Error> {
        let address = address.as_bytes();
        if address.len() != 5 {
            return Err(RF24Error::InvalidAddressWidth);
        }
        self.address.copy_from_slice(address);
        self.rf24
            .write_full_address(Register::RX_ADDR_P0, &self.address)?;
        self.rf24
            .write_full_address(Register::TX_ADDR, &self.address)?;
        Ok(())
    }

    pub fn set_channel(&mut self, channel: u8) -> Result<(), RF24Error> {
        if channel > 127 {
            return Err(RF24Error::InvalidChannel);
        }
        self.channel = channel;
        self.rf24.write_register(Register::RF_CH, channel)?;
        Ok(())
    }

    pub fn configure(&self) -> Result<(), RF24Error> {
        // Disable interrupts, enable CRC, 2 bytes CRC, and set as primary receiver
        let config: u8 = MASK_MAX_RT | MASK_TX_DS | MASK_RX_DR | EN_CRC | CRCO;
        self.rf24.write_register(Register::CONFIG, config)?;

        // Set channel
        self.rf24.write_register(Register::RF_CH, self.channel)?;

        // Set data rate and power level
        let rf_setup: u8 = self.rate as u8 | self.power_level as u8;
        self.rf24.write_register(Register::RF_SETUP, rf_setup)?;

        // Disable auto-ack
        self.rf24.write_register(Register::EN_AA, 0)?;

        // Disable features
        self.rf24.write_register(Register::FEATURE, 0)?;

        // Set payload size
        let payload_size: u8 = 13;
        self.rf24.write_register(Register::RX_PW_P0, payload_size)?;
        self.rf24.write_register(Register::RX_PW_P1, payload_size)?;
        self.rf24.write_register(Register::RX_PW_P2, payload_size)?;
        self.rf24.write_register(Register::RX_PW_P3, payload_size)?;
        self.rf24.write_register(Register::RX_PW_P4, payload_size)?;
        self.rf24.write_register(Register::RX_PW_P5, payload_size)?;

        // Flush RX and TX
        self.rf24.flush_rx()?;
        self.rf24.flush_tx()?;

        // Clear Status
        let status: u8 = RX_DR | TX_DS | MAX_RT;
        self.rf24.write_register(Register::STATUS, status)?;

        // Power up
        self.rf24
            .write_register(Register::CONFIG, config | PWR_UP)?;

        sleep(Duration::from_millis(130));

        Ok(())
    }

    // Send a payload with no ack
    pub fn send(&mut self, payload: &[u8]) -> Result<(), RF24Error> {
        // Wait for TX FIFO to be empty
        'fifo_full: loop {
            let status = self.rf24.read_register(Register::STATUS)?;
            if status & TX_FULL == 0 {
                break 'fifo_full;
            }
            sleep(Duration::from_micros(100));
        }

        // Write payload
        self.rf24.write_payload(payload)?;
        self.rf24.set_ce_high();
        sleep(Duration::from_micros(15));
        self.rf24.set_ce_low();

        // Clear Status
        let status: u8 = RX_DR | TX_DS | MAX_RT;
        self.rf24.write_register(Register::STATUS, status)?;

        Ok(())
    }

    pub fn print_rf_details(&self) {
        self.rf24.print_details().unwrap();
    }
}
