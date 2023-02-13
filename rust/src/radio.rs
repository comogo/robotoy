extern crate nrf24l01;

use std::str::pattern::StrSearcher;

use nrf24l01::{OperatingMode, PALevel, RXConfig, TXConfig, NRF24L01};

enum RadioMode {
    Tx,
    Rx,
}

pub struct Radio {
    device: NRF24L01,
    mode: RadioMode,
    channel: u8,
}

impl Radio {
    pub fn new(channel: u8) -> Self {
        let rx_config = RXConfig {
            channel: 108,
            pa_level: PALevel::Low,
            pipe0_address: *b"radio",
            ..Default::default()
        };

        let ce_pin: u64 = 25;
        let mut device = NRF24L01::new(ce_pin, 0).unwrap();
        device.configure(&OperatingMode::TX(tx_config)).unwrap();
        device.flush_output().unwrap();
        let mode = RadioMode::Tx;

        Radio {
            device,
            mode,
            channel,
        }
    }

    pub fn send(&mut self, message: String) {
        let mut device = self.device;

        let config = TXConfig {
            pa_level: PALevel::Max,
            channel: self.channel,
            max_retries: 0,
            retry_delay: 0,
            pipe0_address: *b"radio",
            data_rate: nrf24l01::DataRate::R250Kbps,
        };

        device.configure(&OperatingMode::TX(config)).unwrap();
        device.flush_output().unwrap();

        device.push(0, message.as_bytes()).unwrap();
        device.send().unwrap();
    }
}
