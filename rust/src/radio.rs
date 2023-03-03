extern crate rppal;

use rppal::{
    gpio,
    gpio::{Gpio, OutputPin},
    spi,
    spi::Segment,
    spi::Spi,
};
use std::cmp;
use std::thread::sleep;
use std::time::Duration;

type Command = u8;
type Register = u8;
type ConfigBit = u8;

const SPI_SPEED: u32 = 10_000_000;

const CMD_R_REGISTER: Command = 0x00;
const CMD_W_REGISTER: Command = 0x20;
const CMD_FLUSH_TX: Command = 0xE1;
const CMD_FLUSH_RX: Command = 0xE2;
const CMD_W_TX_PAYLOAD: Command = 0xA0;
const CMD_W_TX_PAYLOAD_NO_ACK: Command = 0xB0;

const REG_CONFIG: Register = 0x00;
const REG_EN_AA: Register = 0x01;
const REG_EN_RXADDR: Register = 0x02;
const REG_RF_CH: Register = 0x05;
const REG_RF_SETUP: Register = 0x06;
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
    InvalidPipe,
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
struct Status {
    rx_dr: bool,
    tx_ds: bool,
    max_rt: bool,
    rx_p_no: u8,
    tx_full: bool,
}

impl Status {
    fn new(status: u8) -> Self {
        Status {
            rx_dr: (status & 0x40) != 0,
            tx_ds: (status & 0x20) != 0,
            max_rt: (status & 0x10) != 0,
            rx_p_no: (status & 0x0E) >> 1,
            tx_full: (status & 0x01) != 0,
        }
    }
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

        Ok(Device { spi })
    }

    pub fn init(&self) -> Result<(), DeviceError> {
        self.set_rf(DataRate::_250Kbps, Power::_0dBm)?;
        self.set_feature(0)?;
        self.set_dynamic_payload(false)?;
        self.set_auto_ack(true)?;
        self.write_register(REG_EN_RXADDR, 1)?;
        self.set_payload_size(1)?;
        self.set_channel(76)?;
        self.clear_status()?;
        self.flush_rx()?;
        self.flush_tx()?;
        self.write_register(REG_CONFIG, BIT_EN_CRC | BIT_CRCO)?;
        self.power_up()?;

        Ok(())
    }

    pub fn print_registers(&self) -> Result<(), DeviceError> {
        let mut cfg: u8 = 0;

        self.print_status();

        let mut address = self.rx_address(0)?;
        print!("RX_ADDR_P0-1    = 0x");
        for i in address {
            print!("{:02x}", i);
        }

        address = self.rx_address(1)?;
        print!(" 0x");
        for i in address {
            print!("{:02x}", i);
        }

        print!("\nRX_ADDR_P2-5   = ");
        print!(" 0x{:02x}", self.rx_address(2)?[4]);
        print!(" 0x{:02x}", self.rx_address(3)?[4]);
        print!(" 0x{:02x}", self.rx_address(4)?[4]);
        print!(" 0x{:02x}", self.rx_address(5)?[4]);

        println!("");
        address = self.tx_address()?;
        print!("TX_ADDR         = 0x");
        for i in address {
            print!("{:02x}", i);
        }
        println!("");

        (_, cfg) = self.read_register(REG_EN_AA)?;
        println!("EN_AA           = {:#04x}", cfg);

        (_, cfg) = self.read_register(REG_EN_RXADDR)?;
        println!("EN_RXADDR       = {:#04x}", cfg);

        (_, cfg) = self.read_register(REG_RF_CH)?;
        println!("RF_CH           = {:#04x}", cfg);

        (_, cfg) = self.read_register(REG_RF_SETUP)?;
        println!("RF_SETUP        = {:#04x}", cfg);

        (_, cfg) = self.read_register(REG_CONFIG)?;
        println!("CONFIG          = {:#04x}", cfg);

        (_, cfg) = self.read_register(REG_DYNPD)?;
        let (_, feature) = self.read_register(REG_FEATURE)?;
        println!("DYNPD/FEATURE   = {:#04x} {:#04x}", cfg, feature);

        Ok(())
    }

    fn print_status(&self) -> Result<(), DeviceError> {
        let (_, cfg) = self.read_register(REG_STATUS)?;
        print!("STATUS\t\t= {:#04x}", cfg);
        print!(" RX_DR={}", (cfg & 0x40) >> 6);
        print!(" TX_DS={}", (cfg & 0x20) >> 5);
        print!(" MAX_RT={}", (cfg & 0x10) >> 4);
        print!(" RX_P_NO={}", (cfg & 0x0E) >> 1);
        println!(" TX_FULL={}", cfg & 0x01);
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

        let (_, cfg) = self.read_register(REG_RF_SETUP)?;
        let new_config: u8 = (cfg & 0xC0) | rate | power;
        self.write_register(REG_RF_SETUP, new_config)?;
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

    pub fn rx_address(&self, pipe: u8) -> Result<[u8; 5], DeviceError> {
        if pipe > 5 {
            return Err(DeviceError::InvalidPipe);
        }

        let pipe_address: u8 = REG_RX_ADDR_P0 + pipe;

        if pipe < 2 {
            let mut response = [0u8; 6];
            self.command(
                &[CMD_R_REGISTER | (pipe_address), 0u8, 0u8, 0u8, 0u8, 0u8],
                &mut response,
            )?;
            Ok([
                response[5],
                response[4],
                response[3],
                response[2],
                response[1],
            ])
        } else {
            let mut response = [0u8; 3];
            let mut p1_address = self.rx_address(1)?;
            self.command(&[CMD_R_REGISTER | pipe_address, 0u8, 0u8], &mut response)?;
            Ok([
                p1_address[4],
                p1_address[3],
                p1_address[2],
                response[2],
                response[1],
            ])
        }
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

    pub fn tx_address(&self) -> Result<[u8; 5], DeviceError> {
        let mut response = [0u8; 6];
        self.command(
            &[CMD_R_REGISTER | REG_TX_ADDR, 0u8, 0u8, 0u8, 0u8, 0u8],
            &mut response,
        )?;
        Ok([
            response[5],
            response[4],
            response[3],
            response[2],
            response[1],
        ])
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

        self.write_register(REG_RF_CH, channel)?;
        Ok(())
    }

    pub fn channel(&self) -> Result<u8, DeviceError> {
        let (_, channel) = self.read_register(REG_RF_CH)?;
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

    pub fn set_prim_rx(&self, value: bool) -> Result<(), DeviceError> {
        let (_, config) = self.read_register(REG_CONFIG)?;

        if value {
            self.write_register(REG_CONFIG, config | BIT_PRIM_RX)?;
        } else {
            self.write_register(REG_CONFIG, config & !BIT_PRIM_RX)?;
        }

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

    pub fn status(&self) -> Result<Status, DeviceError> {
        let (_, status) = self.read_register(REG_STATUS)?;
        Ok(Status::new(status))
    }

    pub fn write_payload(&self, payload: &[u8], ack: bool) -> Result<Status, DeviceError> {
        if payload.len() > 32 {
            return Err(DeviceError::InvalidPayloadSize);
        }

        let mut response = [0u8; 2];
        let mut command = [0u8; 2];
        command[0] = if ack {
            CMD_W_TX_PAYLOAD
        } else {
            CMD_W_TX_PAYLOAD_NO_ACK
        };
        command[1..payload.len() + 1].copy_from_slice(payload);
        println!("command: {:?}", command);
        self.command(&command, &mut response)?;
        Ok(Status::new(response[0]))
    }
}

#[derive(Debug)]
pub struct Radio {
    device: Device,
    ce_pin: OutputPin,
}

impl Radio {
    pub fn new(ce_pin: u8) -> Result<Self, RadioError> {
        let ce_pin = Gpio::new()
            .map_err(|e| RadioError::GpioError(e))?
            .get(ce_pin)
            .map_err(|e| RadioError::GpioError(e))?
            .into_output_low();

        let device: Device = Device::new().map_err(|e| RadioError::DeviceError(e))?;
        device.init().map_err(|e| RadioError::DeviceError(e))?;

        Ok(Radio { device, ce_pin })
    }

    pub fn print_details(&self) -> Result<(), DeviceError> {
        self.device.print_registers();
        Ok(())
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
        // self.device.set_feature(BIT_EN_DPL | BIT_EN_DYN_ACK)?;
        self.device.set_rf(DataRate::_250Kbps, Power::_0dBm)?;
        // self.device.set_dynamic_payload(true)?;
        // self.device.set_auto_ack(false)?;
        self.device.set_prim_rx(false)?;
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
        self.device.set_prim_rx(false)?;
        Ok(())
    }

    pub fn set_rx_mode(&mut self) -> Result<(), DeviceError> {
        self.device.power_up()?;
        self.device.set_prim_rx(true)?;
        self.ce_pin.set_high();

        Ok(())
    }

    pub fn standby(&mut self) {
        self.ce_pin.set_low();
        sleep(Duration::from_micros(130));
    }

    pub fn start_listening(&mut self) -> Result<(), DeviceError> {
        self.device.set_prim_rx(true)?;
        self.device.clear_status()?;
        self.ce_pin.set_high();
        sleep(Duration::from_micros(130));

        Ok(())
    }

    pub fn stop_listening(&mut self) -> Result<(), DeviceError> {
        self.ce_pin.set_low();
        sleep(Duration::from_micros(130));
        self.device.flush_tx()?;
        self.device.set_prim_rx(false)?;

        Ok(())
    }

    pub fn received_power_detector(&self) -> Result<bool, DeviceError> {
        self.device.rpd()
    }

    pub fn send(&mut self, data: &[u8]) -> Result<(), DeviceError> {
        // Wait for the FIFO to be empty
        'fifo_full: loop {
            let status = self.device.status()?;
            if !status.tx_full {
                break 'fifo_full;
            }
            sleep(Duration::from_micros(15));
        }
        let status = self.device.write_payload(data, false)?;
        self.ce_pin.set_high();
        sleep(Duration::from_micros(15));
        self.ce_pin.set_low();

        println!("Enviado: %{:?} ({:?})", data, status);
        Ok(())
    }
}
