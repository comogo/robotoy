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
use std::cmp;

type Command = u8;
type Register = u8;
type ConfigBit = u8;

const SPI_SPEED: u32 = 10_000_000;

const CMD_R_REGISTER: Command = 0x00;
const CMD_W_REGISTER: Command = 0x20;
const CMD_FLUSH_TX: Command = 0xE1;
const CMD_FLUSH_RX: Command = 0xE2;

const REG_CONFIG: Register = 0x00;
const REG_EN_AA: Register = 0x01;
const REG_EN_RXADDR: Register = 0x02;
const REG_CH: Register = 0x05;
const REG_RF: Register = 0x06;
const REG_RX_ADDR_P0: Register = 0x0A;
const REG_TX_ADDR: Register = 0x10;
const REG_DYNPD: Register = 0x1C;
const REG_FEATURE: Register = 0x1D;
const REG_RPD: Register = 0x09;
const REG_STATUS: Register = 0x07;
const REG_RX_PW_P0: Register = 0x11;

// CONFIG bits
const BIT_PRIM_RX: ConfigBit = 1;
const BIT_PWR_UP: ConfigBit = 2;
const BIT_CRCO: ConfigBit = 4;
const BIT_EN_CRC: ConfigBit = 8;

// RF_SETUP bits
const BIT_RF_PWR: ConfigBit = 6;

// FEATURE bits
const BIT_EN_DYN_ACK: ConfigBit = 1;
const BIT_EN_ACK_PAY: ConfigBit = 2;
const BIT_EN_DPL: ConfigBit = 4;

#[derive(Debug)]
pub enum DeviceError {
    SpiError(spi::Error),
    GpioError(gpio::Error),
    InvalidChannel,
    InvalidPayloadSize,
}

#[derive(Debug)]
pub enum RadioError {
    DeviceError(DeviceError),
    GpioError(gpio::Error),
}

#[derive(Debug)]
pub enum DataRate {
    _250Kbps,
    _1Mbps,
    _2Mbps,
}

#[derive(Debug)]
pub enum Power {
    _18dBm,
    _12dBm,
    _6dBm,
    _0dBm,
}

#[derive(Debug)]
pub enum RadioMode {
    Rx,
    Tx,
}

#[derive(Debug)]
struct Device {
    spi: Spi,
}

impl Device {
    pub fn new() -> Result<Self, DeviceError> {
        let spi: Spi = Spi::new(
            spi::Bus::Spi0,
            spi::SlaveSelect::Ss0,
            SPI_SPEED,
            spi::Mode::Mode0,
        )
        .map_err(|e| DeviceError::SpiError(e))?;

        sleep(Duration::from_millis(5));

        let device = Device { spi };

        device.set_rf(DataRate::_1Mbps, Power::_0dBm)?;
        device.set_feature(0)?;
        device.set_dynamic_payload(false)?;
        device.set_auto_ack(true)?;
        device.write_register(REG_EN_RXADDR, 3)?;
        device.set_payload_size(32)?;
        device.set_channel(76)?;
        device.clear_status()?;
        device.flush_rx()?;
        device.flush_tx()?;
        device.write_register(REG_CONFIG, BIT_EN_CRC | BIT_CRCO)?;
        device.print_status()?;
        device.power_up()?;

        Ok(device)
    }

    fn print_status(&self) -> Result<(), DeviceError> {
        let (_, cfg) = self.read_register(REG_CONFIG)?;
        println!("MASK_RX_DR: {}", (cfg & 0x40) >> 6);
        println!("MASK_TX_DS: {}", (cfg & 0x20) >> 5);
        println!("MASK_MAX_RT: {}", (cfg & 0x10) >> 4);
        println!("EN_CRC: {}", (cfg & 0x08) >> 3);
        println!("CRCO: {}", (cfg & 0x04) >> 2);
        println!("PWR_UP: {}", (cfg & 0x02) >> 1);
        println!("PRIM_RX: {}", cfg & 0x01);
        Ok(())
    }

    fn command(&self, data_out: &[u8], data_in: &mut [u8]) -> Result<(), DeviceError> {
        self.spi
            .transfer(data_in, data_out)
            .map_err(|e| DeviceError::SpiError(e))?;

        Ok(())
    }

    fn write_register(&self, register: Register, value: u8) -> Result<(), DeviceError> {
        let mut response = [0u8; 2];
        self.command(&[CMD_W_REGISTER | register, value], &mut response)
    }

    fn read_register(&self, register: Register) -> Result<(u8, u8), DeviceError> {
        let mut response = [0u8; 2];
        self.command(&[CMD_R_REGISTER | register, 0], &mut response)?;

        Ok((response[0], response[1]))
    }

    pub fn flush_tx(&self) -> Result<(), DeviceError> {
        let mut response = [0u8; 1];
        self.command(&[CMD_FLUSH_TX], &mut response)?;
        Ok(())
    }

    pub fn flush_rx(&self) -> Result<(), DeviceError> {
        let mut response = [0u8; 1];
        self.command(&[CMD_FLUSH_RX], &mut response)?;
        Ok(())
    }

    pub fn set_payload_size(&self, size: u8) -> Result<(), DeviceError> {
        if size > 32 {
            return Err(DeviceError::InvalidPayloadSize);
        }

        for i in 0..5 {
            self.write_register(REG_RX_PW_P0 + i, size)?;
        }
        Ok(())
    }

    /// Set the data rate and the power level.
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

    /// Function used to enable/disable the dynamic payload length.
    pub fn set_dynamic_payload(&self, enable: bool) -> Result<(), DeviceError> {
        let value: u8 = if enable { 0b0011_1111 } else { 0b0000_0000 };
        self.write_register(REG_DYNPD, value)
    }

    /// Enable only the pipe 0 and set its address.
    /// The address is 5 bytes long.
    pub fn set_rx_address(&self, address: [u8; 5]) -> Result<(), DeviceError> {
        let mut response = [0u8; 6];
        self.command(
            &[
                CMD_W_REGISTER | REG_RX_ADDR_P0,
                address[4],
                address[3],
                address[2],
                address[1],
                address[0],
            ],
            &mut response,
        )?;
        self.write_register(CMD_W_REGISTER | REG_EN_RXADDR, 0x03)?;
        Ok(())
    }

    /// Set the address used for the transmission.
    /// The address is 5 bytes long.
    pub fn set_tx_address(&self, address: [u8; 5]) -> Result<(), DeviceError> {
        let mut response = [0u8; 6];
        self.command(
            &[
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

    /// Enable or disable the auto acknowledgment for all pipes.
    pub fn set_auto_ack(&self, enable: bool) -> Result<(), DeviceError> {
        let mut response = [0u8; 2];
        let value: u8 = if enable { 0b0011_1111 } else { 0b0000_0000 };
        self.command(&[CMD_W_REGISTER | REG_EN_AA, value], &mut response)?;
        Ok(())
    }

    /// Set the channel used for the communication.
    pub fn set_channel(&self, channel: u8) -> Result<(), DeviceError> {
        if channel > 125 {
            return Err(DeviceError::InvalidChannel);
        }

        self.write_register(REG_CH, channel)?;
        Ok(())
    }

    pub fn channel(&self) -> Result<u8, DeviceError> {
        let (_, channel) = self.read_register(REG_CH)?;
        Ok(channel)
    }

    /// Configure the NRF24L01+ features.
    /// The features are:
    /// - Dynamic payload length
    /// - Payload with ACK
    /// - Dynamic ACK
    pub fn set_feature(&self, value: u8) -> Result<(), DeviceError> {
        self.write_register(REG_FEATURE, value)?;
        Ok(())
    }

    /// Set the NRF24L01+ in TX mode, PWR_UP = 0, PRIM_RX = 0.
    pub fn set_tx_mode(&self) -> Result<(), DeviceError> {
        self.write_register(REG_CONFIG, 0b0000_010)?;
        Ok(())
    }

    /// Set the NRF24L01+ in RX mode, PWR_UP = 1, PRIM_RX = 1.
    pub fn set_rx_mode(&self) -> Result<(), DeviceError> {
        self.write_register(REG_CONFIG, 0b0000_011)?;
        Ok(())
    }

    pub fn disable_prim_rx(&self) -> Result<(), DeviceError> {
        let (_, config) = self.read_register(REG_CONFIG)?;
        self.write_register(REG_CONFIG, config & !BIT_PRIM_RX)?;
        Ok(())
    }

    pub fn power_up(&self) -> Result<(), DeviceError> {
        let (_, config) = self.read_register(REG_CONFIG)?;
        self.write_register(REG_CONFIG, config | 0b0000_010)?;
        sleep(Duration::from_millis(5));
        Ok(())
    }

    pub fn power_down(&self) -> Result<(), DeviceError> {
        let (_, config) = self.read_register(REG_CONFIG)?;
        self.write_register(REG_CONFIG, config & 0b111_101)?;
        Ok(())
    }

    /// Received Power Detector.
    pub fn rpd(&self) -> Result<bool, DeviceError> {
        let (_, value) = self.read_register(REG_RPD)?;
        Ok(value == 1)
    }

    pub fn clear_status(&self) -> Result<(), DeviceError> {
        self.write_register(REG_STATUS, 0b0111_0000)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Radio {
    device: Device,
    ce_pin: OutputPin,
    mode: RadioMode,
}

impl Radio {
    pub fn new(ce_pin: u8) -> Result<Self, RadioError> {
        let ce_pin = Gpio::new()
            .map_err(|e| RadioError::GpioError(e))?
            .get(ce_pin)
            .map_err(|e| RadioError::GpioError(e))?
            .into_output_low();

        let device: Device = Device::new().map_err(|e| RadioError::DeviceError(e))?;

        Ok(Radio {
            device,
            ce_pin,
            mode: RadioMode::Rx,
        })
    }

    pub fn scan(&mut self) -> Result<(), DeviceError> {
        self.device.set_auto_ack(false)?;
        self.start_listening()?;
        self.stop_listening()?;
        self.device.power_down()?;

        let mut data: [u8; 127] = [0; 127];

        'scanner: loop {
            data = [0; 127];

            for run in 1..100 {
                for i in 0..126 {
                    self.device.set_channel(i)?;
                    self.start_listening()?;
                    sleep(Duration::from_micros(130));
                    self.stop_listening()?;

                    if self.device.rpd()? { 
                        data[i as usize] += 1;
                    }
                }
            }

            for rssi in data.iter() {
                if rssi > &0 {
                    print!("{:01x}", cmp::min(0xf, (rssi & 0xf)));
                } else {
                    print!("-");
                }
            }
            println!("");
        }
        Ok(())
    }

    /// Configure the NRF24L01+ to use 250Kbps data rate, 0dBm power,
    /// dynamic payload, disable auto ack and TX mode.
    pub fn setup(&self) -> Result<(), DeviceError> {
        self.device
            .set_feature(BIT_EN_DPL | BIT_EN_DYN_ACK)?;
        self.device.set_rf(DataRate::_250Kbps, Power::_0dBm)?;
        self.device.set_dynamic_payload(true)?;
        self.device.set_auto_ack(false)?;
        self.device.set_tx_mode()?;
        Ok(())
    }

    pub fn set_channel(&self, channel: u8) -> Result<(), DeviceError> {
        self.device.set_channel(channel)?;
        Ok(())
    }

    pub fn channel(&self) -> Result<u8, DeviceError> {
        self.device.channel()
    }

    pub fn set_address(&self, tx_address: [u8; 5], rx_address: [u8; 5]) -> Result<(), DeviceError> {
        self.device.set_rx_address(rx_address)?;
        self.device.set_tx_address(tx_address)?;
        Ok(())
    }

    pub fn set_tx_mode(&mut self) -> Result<(), DeviceError> {
        self.ce_pin.set_low();
        sleep(Duration::from_micros(130));
        self.device.flush_tx()?;
        self.device.set_tx_mode()?;
        self.mode = RadioMode::Tx;
        Ok(())
    }

    pub fn set_rx_mode(&mut self) -> Result<(), DeviceError> {
        self.device.power_up()?;
        self.mode = RadioMode::Rx;
        self.device.set_rx_mode()?;
        self.ce_pin.set_high();

        Ok(())
    }

    pub fn standby(&mut self) {
        self.ce_pin.set_low();
        sleep(Duration::from_micros(130));
    }

    pub fn start_listening(&mut self) -> Result<(), DeviceError> {
        self.device.set_rx_mode()?;
        self.device.clear_status()?;
        self.ce_pin.set_high();
        sleep(Duration::from_micros(130));

        Ok(())
    }

    pub fn stop_listening(&mut self) -> Result<(), DeviceError> {
        self.ce_pin.set_low();
        sleep(Duration::from_micros(130));
        self.device.flush_tx()?;
        self.device.disable_prim_rx()?;

        Ok(())
    }

    pub fn received_power_detector(&self) -> Result<bool, DeviceError> {
        self.device.rpd()
    }
}
