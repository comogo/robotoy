extern crate rppal;

use rppal::{
    gpio,
    gpio::{Gpio, OutputPin},
    spi,
    spi::Spi,
};

type Command = u8;
type Register = u8;

const CMD_R_REGISTER: Command = 0x00;
const CMD_W_REGISTER: Command = 0x20;

const REG_CONFIG: Register = 0x00;
const REG_EN_AA: Register = 0x01;
const REG_CH: Register = 0x05;
const REG_RF: Register = 0x06;
const REG_RX_ADDR_P0: Register = 0x0A;
const REG_TX_ADDR: Register = 0x10;
const REG_DYNPD: Register = 0x1C;
const REG_FEATURE: Register = 0x1D;

#[derive(Debug)]
pub enum DeviceError {
    SpiError(spi::Error),
    InvalidChannel,
}

#[derive(Debug)]
pub enum RadioError {
    DeviceError(DeviceError),
    GpioError(gpio::Error),
}

enum DataRate {
    _250Kbps,
    _1Mbps,
    _2Mbps,
}

enum Power {
    _18dBm,
    _12dBm,
    _6dBm,
    _0dBm,
}

#[derive(Debug)]
struct Device {
    spi: Spi,
    ce: gpio::OutputPin,
    csn: gpio::OutputPin,
}

impl Device {
    pub fn new(ce_pin: u8, csn_pin: u8) -> Result<Self, DeviceError> {
        let gpio: Gpio = Gpio::new().unwrap();
        let ce: gpio::OutputPin = gpio.get(ce_pin).unwrap().into_output();
        let csn: gpio::OutputPin = gpio.get(csn_pin).unwrap().into_output();
        let spi: Spi = Spi::new(
            spi::Bus::Spi0,
            spi::SlaveSelect::Ss0,
            10_000_000,
            spi::Mode::Mode0,
        )
        .map_err(|e| DeviceError::SpiError(e))?;

        Ok(Device { spi, ce, csn })
    }

    fn command(&self, data_in: &mut [u8], data_out: &mut [u8]) -> Result<usize, DeviceError> {
        self.spi
            .transfer(data_in, data_out)
            .map_err(|e| DeviceError::SpiError(e))
    }

    fn write_register(&self, register: Register, value: u8) -> Result<usize, DeviceError> {
        let mut response = [0u8; 2];
        self.command(&mut [CMD_W_REGISTER | register, value], &mut response)
    }

    fn read_register(&self, register: Register) -> Result<(u8, u8), DeviceError> {
        let mut response = [0u8; 2];
        self.command(&mut [CMD_R_REGISTER | register, 0], &mut response)?;

        Ok((response[0], response[1]))
    }

    pub fn set_rf(&self, data_rate: DataRate, power_level: Power) -> Result<(), DeviceError> {
        let rate: u8 = match data_rate {
            DataRate::_250Kbps => 0b0010_0000,
            DataRate::_1Mbps => 0b0000_0000,
            DataRate::_2Mbps => 0b0000_1000,
        };

        let power: u8 = match power_level {
            Power::_18dBm => 0b0000_0000,
            Power::_12dBm => 0b0000_0010,
            Power::_6dBm => 0b0000_0100,
            Power::_0dBm => 0b0000_0110,
        };

        self.write_register(REG_RF, rate | power)?;
        Ok(())
    }

    pub fn set_rx_address(&self, address: [u8; 5]) -> Result<(), DeviceError> {
        let mut response = [0u8; 6];
        self.command(
            &mut [
                CMD_W_REGISTER | REG_RX_ADDR_P0,
                address[4],
                address[3],
                address[2],
                address[1],
                address[0],
            ],
            &mut response,
        )?;
        Ok(())
    }

    pub fn set_dynamic_payload(&self, enable: bool) -> Result<(), DeviceError> {
        let mut response = [0u8; 2];
        let value: u8 = if enable { 0b0011_1111 } else { 0b0000_0000 };
        self.command(&mut [CMD_W_REGISTER | REG_DYNPD, value], &mut response)?;
        Ok(())
    }

    pub fn set_tx_address(&self, address: [u8; 5]) -> Result<(), DeviceError> {
        let mut response = [0u8; 6];
        self.command(
            &mut [
                CMD_W_REGISTER | REG_TX_ADDR,
                address[4],
                address[3],
                address[2],
                address[1],
                address[0],
            ],
            &mut response,
        )?;
        Ok(())
    }

    pub fn set_auto_ack(&self, enable: bool) -> Result<(), DeviceError> {
        let mut response = [0u8; 2];
        let value: u8 = if enable { 0b0011_1111 } else { 0b0000_0000 };
        self.command(&mut [CMD_W_REGISTER | REG_EN_AA, value], &mut response)?;
        Ok(())
    }

    pub fn set_channel(&self, channel: u8) -> Result<(), DeviceError> {
        if channel > 125 {
            return Err(DeviceError::InvalidChannel);
        }

        self.write_register(REG_CH, channel)?;
        Ok(())
    }

    pub fn set_feature(&self, value: u8) -> Result<(), DeviceError> {
        self.write_register(REG_FEATURE, value)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Radio {
    device: Device,
    ce_pin: OutputPin,
    csn_pin: OutputPin,
}

impl Radio {
    pub fn new(ce_pin: u8, csn_pin: u8) -> Result<Self, RadioError> {
        let device: Device =
            Device::new(ce_pin, csn_pin).map_err(|e| RadioError::DeviceError(e))?;

        let ce_pin = Gpio::new()
            .map_err(|e| RadioError::GpioError(e))?
            .get(ce_pin)
            .map_err(|e| RadioError::GpioError(e))?
            .into_output();

        let csn_pin = Gpio::new()
            .map_err(|e| RadioError::GpioError(e))?
            .get(csn_pin)
            .map_err(|e| RadioError::GpioError(e))?
            .into_output();

        Ok(Radio {
            device,
            ce_pin,
            csn_pin,
        })
    }

    pub fn setup(&self) -> Result<(), DeviceError> {
        self.device.set_rf(DataRate::_250Kbps, Power::_0dBm)?;
        self.device.set_dynamic_payload(true)?;
        self.device.set_auto_ack(false)?;

        // Enable dynamic payload length and NOACK
        // EN_DPL = 1
        // EN_ACK_PAY = 0
        // EN_DYN_ACK = 1
        self.device.set_feature(0b0000_0101)?;
        Ok(())
    }

    pub fn set_channel(&self, channel: u8) {
        self.device.set_channel(channel);
    }

    pub fn set_address(&self, tx_address: [u8; 5], rx_address: [u8; 5]) {
        self.device.set_rx_address(rx_address);
        self.device.set_tx_address(tx_address);
    }
}
