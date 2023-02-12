extern crate nrf24l01;

use nrf24l01::{OperatingMode, PALevel, RXConfig, TXConfig, NRF24L01};

enum RadioMode {
    TX,
    RX,
}

pub struct Radio {
    device: NRF24L01,
    tx_config: TXConfig,
    rx_config: RXConfig,
    mode: RadioMode,
}

impl Radio {
    pub fn new(channel: u8) -> Self {
        let tx_config = TXConfig {
            pa_level: PALevel::Max,
            channel: channel,
            max_retries: 0,
            retry_delay: 0,
            pipe0_address: *b"radio",
            data_rate: nrf24l01::DataRate::R250Kbps,
        };

        let rx_config = RXConfig {
            channel: 108,
            pa_level: PALevel::Low,
            pipe0_address: *b"radio",
            ..Default::default()
        };

        let ce_pin: u64 = 25;
        let mut device = NRF24L01::new(ce_pin, 0).unwrap();

        OperatingMode::Radio {
            device,
            tx_config,
            rx_config,
            mode,
        }
    }
}
