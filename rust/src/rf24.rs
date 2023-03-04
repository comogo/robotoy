extern crate rppal;

use rppal::{spi, spi::Spi};
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
pub enum Power {
    _18dBm = 0x00,
    _12dBm = 0x02,
    _6dBm = 0x04,
    _0dBm = 0x06,
}

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Power::_18dBm => write!(f, "-18dBm"),
            Power::_12dBm => write!(f, "-12dBm"),
            Power::_6dBm => write!(f, "-6dBm"),
            Power::_0dBm => write!(f, "0dBm"),
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
    SpiError(spi::Error),
    InvalidChannel,
    InvalidPayloadSize,
    InvalidPipe,
    InvalidAddressWidth,
}

#[derive(Debug)]
pub struct RF24 {
    spi: Spi,
}

impl RF24 {
    pub fn new() -> Result<RF24, RF24Error> {
        let spi: Spi = Spi::new(
            spi::Bus::Spi0,
            spi::SlaveSelect::Ss0,
            SPI_SPEED,
            spi::Mode::Mode0,
        )
        .map_err(|e| RF24Error::SpiError(e))?;

        sleep(Duration::from_millis(5));

        Ok(RF24 { spi })
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
    fn read_register(&self, reg: Register, size: usize, data: &mut [u8]) -> Result<(), RF24Error> {
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
    fn write_register(&self, reg: Register, value: &[u8]) -> Result<(), RF24Error> {
        let size = value.len();
        let mut data_out: Vec<u8> = vec![0; size + 1];
        let mut data_in: Vec<u8> = vec![0; size + 1];
        data_out[0] = Command::W_REGISTER as u8 | reg as u8;
        data_out[1..].copy_from_slice(value);
        self.command(&data_out, &mut data_in)?;
        Ok(())
    }

    /// Set the Configuration Register (CONFIG).
    pub fn set_config(&self, value: u8) -> Result<(), RF24Error> {
        self.write_register(Register::CONFIG, &[value])
    }

    /// Reads the Configuration Register (CONFIG).
    pub fn config(&self) -> Result<u8, RF24Error> {
        let mut data = [0u8];
        self.read_register(Register::CONFIG, 1, &mut data)?;
        Ok(data[0])
    }

    /// Set the Enable Auto Acknowledgment Register (EN_AA).
    pub fn set_en_aa(&self, value: u8) -> Result<(), RF24Error> {
        self.write_register(Register::EN_AA, &[value])
    }

    /// Reads the Enable Auto Acknowledgment Register (EN_AA).
    pub fn en_aa(&self) -> Result<u8, RF24Error> {
        let mut data = [0u8];
        self.read_register(Register::EN_AA, 1, &mut data)?;
        Ok(data[0])
    }

    /// Set the Enabled RX Addresses Register (EN_RXADDR).
    pub fn set_en_rxaddr(&self, value: u8) -> Result<(), RF24Error> {
        self.write_register(Register::EN_RXADDR, &[value])
    }

    /// Reads the Enabled RX Addresses Register (EN_RXADDR).
    pub fn en_rxaddr(&self) -> Result<u8, RF24Error> {
        let mut data = [0u8];
        self.read_register(Register::EN_RXADDR, 1, &mut data)?;
        Ok(data[0])
    }

    /// Set the Setup of Address Widths Register (SETUP_AW).
    pub fn set_setup_aw(&self, value: u8) -> Result<(), RF24Error> {
        if value == 0 || value > 3 {
            return Err(RF24Error::InvalidAddressWidth);
        }
        self.write_register(Register::SETUP_AW, &[value])
    }

    /// Reads the Setup of Address Widths Register (SETUP_AW).
    pub fn setup_aw(&self) -> Result<u8, RF24Error> {
        let mut data = [0u8];
        self.read_register(Register::SETUP_AW, 1, &mut data)?;
        Ok(data[0])
    }

    /// Set the Setup of Automatic Retransmission Register (SETUP_RETR).
    pub fn set_setup_retr(&self, value: u8) -> Result<(), RF24Error> {
        self.write_register(Register::SETUP_RETR, &[value])
    }

    /// Reads the Setup of Automatic Retransmission Register (SETUP_RETR).
    pub fn setup_retr(&self) -> Result<u8, RF24Error> {
        let mut data = [0u8];
        self.read_register(Register::SETUP_RETR, 1, &mut data)?;
        Ok(data[0])
    }

    /// Set the channel frequency.
    ///
    /// The channel varies from 0 to 127.
    pub fn set_rf_ch(&self, channel: u8) -> Result<(), RF24Error> {
        if channel > 127 {
            return Err(RF24Error::InvalidChannel);
        }
        self.write_register(Register::RF_CH, &[channel])
    }

    /// Get the channel frequency stored in the NRF24L01+ module.
    pub fn channel(&self) -> Result<u8, RF24Error> {
        let mut data: [u8; 1] = [0];
        self.read_register(Register::RF_CH, 1, &mut data)?;
        Ok(data[0])
    }

    pub fn set_rf_setup(&self, data_rate: DataRate, power: Power) -> Result<(), RF24Error> {
        let mut data = [0u8];
        self.read_register(Register::RF_SETUP, 1, &mut data)?;
        data[0] &= 0b1101_0001;
        data[0] |= data_rate as u8;
        data[0] |= power as u8;
        self.write_register(Register::RF_SETUP, &data)
    }

    pub fn rf_setup(&self) -> Result<u8, RF24Error> {
        let mut data = [0u8];
        self.read_register(Register::RF_SETUP, 1, &mut data)?;
        Ok(data[0])
    }

    pub fn test(&self) -> Result<(), RF24Error> {
        println!("Channel: {:02x}", self.channel()?);
        println!("RF_SETUP: {:02x}", self.rf_setup()?);

        self.set_channel(120)?;
        self.set_rf_setup(DataRate::_250Kbps, Power::_18dBm)?;

        println!("Channel: {:02x}", self.channel()?);
        println!("RF_SETUP: {:02x}", self.rf_setup()?);
        Ok(())
    }
}
