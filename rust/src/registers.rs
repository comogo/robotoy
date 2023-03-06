extern crate rppal;

use std::str::FromStr;

use rppal::{
    gpio,
    gpio::{Gpio, OutputPin},
    spi,
    spi::Spi,
};

const SPI_SPEED_MHZ: u32 = 10_000_000;

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

fn is_bit_set(value: u8, bit: u8) -> bool {
    value & bit != 0
}

enum RegisterError {
    InvalidValue(String),
}

/// The Configuration Register.
///
/// **mask_rx_dr** - *default: false* - Mask interrupt caused by RX_DR
///
/// Mask interrupt caused by RX_DR
///
///     - true: Interrupt not reflected on the IRQ pin
///     - false: Reflect RX_DR as active low interrupt on the IRQ pin
///
/// **mask_tx_ds** - *default: false* - Mask interrupt caused by TX_DS
///
///     - true: Interrupt not reflected on the IRQ pin
///     - false: Reflect TX_DS as active low interrupt on the IRQ pin
///
/// **mask_max_rt** - *default: false* - Mask interrupt caused by MAX_RT
///
///     - true: Interrupt not reflected on the IRQ pin
///     - false: Reflect MAX_RT as active low interrupt on the IRQ pin
///
/// **en_crc** - *default: true* - Enable CRC. Forced high if one of the bits in the EN_AA is high
///
/// **crc0** - *default: false* - CRC encoding scheme
///
///     - true: 2 byte CRC
///     - false: 1 byte CRC
///
/// **pwr_up** - *default: false* - Power up/down
///
///     - true: Power up
///     - false: Power down
///
/// **prim_rx** - *default: false* - RX/TX control
///
///     - true: PRX - Primary receiver
///     - false: PTX - Primary transmitter
struct ConfigRegister {
    mask_max_rt: bool,
    mask_tx_ds: bool,
    mask_rx_dr: bool,
    en_crc: bool,
    crc0: bool,
    pwr_up: bool,
    prim_rx: bool,
}

impl ConfigRegister {
    /// Creates a new ConfigRegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a ConfigRegister.
    pub fn new(value: u8) -> Self {
        Self {
            mask_rx_dr: is_bit_set(value, MASK_RX_DR),
            mask_tx_ds: is_bit_set(value, MASK_TX_DS),
            mask_max_rt: is_bit_set(value, MASK_MAX_RT),
            en_crc: is_bit_set(value, EN_CRC),
            crc0: is_bit_set(value, CRCO),
            pwr_up: is_bit_set(value, PWR_UP),
            prim_rx: is_bit_set(value, PRIM_RX),
        }
    }

    /// Sets the value of MASK_RX_DR bit.
    ///
    /// This bit mask interrupt caused by RX_DR.
    ///
    /// - `true`: Interrupt not reflected on the IRQ pin
    /// - `false`: Reflect RX_DR as active low interrupt on the IRQ pin
    pub fn set_mask_rx_dr(&mut self, value: bool) {
        self.mask_rx_dr = value;
    }

    pub fn mask_rx_dr(&self) -> bool {
        self.mask_rx_dr
    }

    /// Sets the value of MASK_TX_DS bit.
    ///
    /// Mask interrupt caused by TX_DS
    ///
    /// - `true`: Interrupt not reflected on the IRQ pin
    /// - `false`: Reflect TX_DS as active low interrupt on the IRQ pin
    pub fn set_mask_tx_ds(&mut self, value: bool) {
        self.mask_tx_ds = value;
    }

    pub fn mask_tx_ds(&self) -> bool {
        self.mask_tx_ds
    }

    /// Sets the value of MASK_MAX_RT bit.
    ///
    /// Mask interrupt caused by MAX_RT
    ///
    /// - `true`: Interrupt not reflected on the IRQ pin
    /// - `false`: Reflect MAX_RT as active low interrupt on the IRQ pin
    pub fn set_mask_max_rt(&mut self, value: bool) {
        self.mask_max_rt = value;
    }

    pub fn mask_max_rt(&self) -> bool {
        self.mask_max_rt
    }

    /// Sets the value of EN_CRC bit.
    ///
    /// Enable/Disable CRC.
    ///
    /// Forced high if one of the bits in the EN_AA is high
    pub fn set_en_crc(&mut self, value: bool) {
        self.en_crc = value;
    }

    pub fn en_crc(&self) -> bool {
        self.en_crc
    }

    /// Sets the value of CRCO bit.
    ///
    /// CRC encoding scheme
    ///
    /// - `true`: 2 byte CRC
    /// - `false`: 1 byte CRC
    pub fn set_crc0(&mut self, value: bool) {
        self.crc0 = value;
    }

    pub fn crc0(&self) -> bool {
        self.crc0
    }

    /// Sets the value of PWR_UP bit.
    ///
    /// Power up/down
    ///
    /// - `true`: Power up
    /// - `false`: Power down
    pub fn set_pwr_up(&mut self, value: bool) {
        self.pwr_up = value;
    }

    pub fn pwr_up(&self) -> bool {
        self.pwr_up
    }

    /// Sets the value of PRIM_RX bit.
    ///
    /// RX/TX control
    ///
    /// - `true`: PRX - Primary receiver
    /// - `false`: PTX - Primary transmitter
    pub fn set_prim_rx(&mut self, value: bool) {
        self.prim_rx = value;
    }

    pub fn prim_rx(&self) -> bool {
        self.prim_rx
    }

    /// Converts the ConfigRegister to a u8 value.
    ///
    /// The function should be used to write the value to the device.
    pub fn to_u8(&self) -> u8 {
        let mut value = 0u8;
        if self.mask_rx_dr {
            value |= MASK_RX_DR;
        }
        if self.mask_tx_ds {
            value |= MASK_TX_DS;
        }
        if self.mask_max_rt {
            value |= MASK_MAX_RT;
        }
        if self.en_crc {
            value |= EN_CRC;
        }
        if self.crc0 {
            value |= CRCO;
        }
        if self.pwr_up {
            value |= PWR_UP;
        }
        if self.prim_rx {
            value |= PRIM_RX;
        }
        value
    }
}

/// The Enable Auto Acknowledgment Register - Enhanced ShockBurst™.
///
/// - **en_aa_p5** - *default: true* - Enable auto acknowledgment data pipe 5
/// - **en_aa_p4** - *default: true* - Enable auto acknowledgment data pipe 4
/// - **en_aa_p3** - *default: true* - Enable auto acknowledgment data pipe 3
/// - **en_aa_p2** - *default: true* - Enable auto acknowledgment data pipe 2
/// - **en_aa_p1** - *default: true* - Enable auto acknowledgment data pipe 1
/// - **en_aa_p0** - *default: true* - Enable auto acknowledgment data pipe 0
struct EnableAutoAcknowledgmentRegister {
    en_aa_p5: bool,
    en_aa_p4: bool,
    en_aa_p3: bool,
    en_aa_p2: bool,
    en_aa_p1: bool,
    en_aa_p0: bool,
}

impl EnableAutoAcknowledgmentRegister {
    /// Creates a new EnAARegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a EnAARegister.
    pub fn new(value: u8) -> Self {
        Self {
            en_aa_p5: is_bit_set(value, ENAA_P5),
            en_aa_p4: is_bit_set(value, ENAA_P4),
            en_aa_p3: is_bit_set(value, ENAA_P3),
            en_aa_p2: is_bit_set(value, ENAA_P2),
            en_aa_p1: is_bit_set(value, ENAA_P1),
            en_aa_p0: is_bit_set(value, ENAA_P0),
        }
    }

    /// Converts the EnAARegister to a u8 value.
    ///
    /// The function should be used to write the value to the device.
    pub fn to_u8(&self) -> u8 {
        let mut value = 0u8;
        if self.en_aa_p5 {
            value |= ENAA_P5;
        }
        if self.en_aa_p4 {
            value |= ENAA_P4;
        }
        if self.en_aa_p3 {
            value |= ENAA_P3;
        }
        if self.en_aa_p2 {
            value |= ENAA_P2;
        }
        if self.en_aa_p1 {
            value |= ENAA_P1;
        }
        if self.en_aa_p0 {
            value |= ENAA_P0;
        }
        value
    }
}

/// The Enable RX Addresses Register.
///
/// - **erx_p5** - *default: false* - Enable data pipe 5
/// - **erx_p4** - *default: false* - Enable data pipe 4
/// - **erx_p3** - *default: false* - Enable data pipe 3
/// - **erx_p2** - *default: false* - Enable data pipe 2
/// - **erx_p1** - *default: true* - Enable data pipe 1
/// - **erx_p0** - *default: true* - Enable data pipe 0
struct EnableRXAddressesRegister {
    erx_p5: bool,
    erx_p4: bool,
    erx_p3: bool,
    erx_p2: bool,
    erx_p1: bool,
    erx_p0: bool,
}

impl EnableRXAddressesRegister {
    /// Creates a new EnableRXAddressesRegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a EnableRXAddressesRegister.
    pub fn new(value: u8) -> Self {
        Self {
            erx_p5: is_bit_set(value, ERX_P5),
            erx_p4: is_bit_set(value, ERX_P4),
            erx_p3: is_bit_set(value, ERX_P3),
            erx_p2: is_bit_set(value, ERX_P2),
            erx_p1: is_bit_set(value, ERX_P1),
            erx_p0: is_bit_set(value, ERX_P0),
        }
    }

    /// Converts the EnRXAddressRegister to a u8 value.
    ///
    /// The function should be used to write the value to the device.
    fn to_u8(&self) -> u8 {
        let mut value = 0u8;
        if self.erx_p5 {
            value |= ERX_P5;
        }
        if self.erx_p4 {
            value |= ERX_P4;
        }
        if self.erx_p3 {
            value |= ERX_P3;
        }
        if self.erx_p2 {
            value |= ERX_P2;
        }
        if self.erx_p1 {
            value |= ERX_P1;
        }
        if self.erx_p0 {
            value |= ERX_P0;
        }
        value
    }
}

/// The Setup Address Width Register.
///
/// **aw** - *default: 3* - Address width
///
///     - 0b00: Illegal
///     - 0b01: 3 bytes
///     - 0b10: 4 bytes
///     - 0b11: 5 bytes
struct SetupAddressWidthsRegister {
    aw: u8,
}

impl SetupAddressWidthsRegister {
    /// Creates a new SetupAddressWidthsRegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a SetupAddressWidthsRegister.
    pub fn new(value: u8) -> Self {
        Self { aw: value & 0b11 }
    }

    /// Converts the SetupAddressWidthsRegister to a u8 value.
    ///
    /// The function should be used to write the value to the device.
    pub fn to_u8(&self) -> u8 {
        self.aw
    }
}

/// The SetupRetransmitsRegister contains the auto retransmit delay and the number of
/// retransmits.
///
/// - ard: Auto Retransmit Delay
/// - arc: Auto Retransmit Count
///
/// The delay is calculated by the following formula:
///
/// delay = (ard + 1) * 250us
///
///     - 0000: Wait 250us
///     - 0001: Wait 500us
///     - 0010: Wait 750us
///     - ...
///     - 1111: Wait 4000us
///
///
/// The number of retransmits is calculated by the following way:
///
/// retransmits = arc
///
///     - 0000: Re-transmit disabled
///     - 0001: Up to 1 re-transmit on fail of AA
///     - ...
///     - 1111: Up to 15 re-transmits on fail of AA
///
/// The default values are 0x00 which means a delay of 250us and 3 retransmits.
struct SetupRetransmitsRegister {
    ard: u8,
    arc: u8,
}

impl SetupRetransmitsRegister {
    /// Creates a new SetupRetransmitsRegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a SetupRetransmitsRegister.
    pub fn new(value: u8) -> SetupRetransmitsRegister {
        SetupRetransmitsRegister {
            arc: value >> 4,
            ard: value & 0xF,
        }
    }

    /// Converts the SetupRetransmitsRegister to a u8 value.
    ///
    /// The function should be used to write the value to the device.
    pub fn to_u8(&self) -> u8 {
        (self.arc << 4) | self.ard
    }
}

/// The RF Channel Register.
///
/// The frequency is calculated by the following formula:
///
///     frequency = 2400 + rf_ch
///
/// Only the 7 LSB are used.
///
/// **rf_ch** - *default: 2* - RF Channel
///
///     - 0b000_0000 - 2400 MHz
///     - 0b000_0001 - 2401 MHz
///     - ...
///     - 0b111_1111 - 2525 MHz
struct RFChannelRegister {
    rf_ch: u8,
}

impl RFChannelRegister {
    /// Creates a new RFChannelRegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a RFChannelRegister.
    pub fn new(value: u8) -> RFChannelRegister {
        RFChannelRegister {
            rf_ch: value & 0x7F,
        }
    }

    /// Converts the RFChannelRegister to a u8 value.
    ///
    /// The function should be used to write the value to the device.
    pub fn to_u8(&self) -> u8 {
        self.rf_ch
    }
}

/// The RF Setup Register.
///
/// **cont_wave** - *default: false* - Enables continuous carrier transmit when true
///
/// **rf_dr** - *default: 0x00* - Data rate (250kbps, 1Mbps, 2Mbps)
///
///     - 0b00 - 1Mbps
///     - 0b01 - 2Mbps
///     - 0b10 - 250kbps
///
/// **rf_pwr** - *default: 0x03* - RF output power in TX mode
///
///     - 0b00 - -18dBm
///     - 0b01 - -12dBm
///     - 0b10 - -6dBm
///     - 0b11 - 0dBm
struct RFSetupRegister {
    cont_wave: bool,
    rf_dr: u8,
    rf_pwr: u8,
}

impl RFSetupRegister {
    /// Creates a new RFSetupRegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a RFSetupRegister.
    pub fn new(value: u8) -> Self {
        Self {
            cont_wave: is_bit_set(value, CONT_WAVE),
            rf_dr: (((value >> 5) << 1) | (value >> 3)) & 3,
            rf_pwr: (value & RF_PWR) >> 1,
        }
    }

    /// Converts the RFSetupRegister to a u8 value.
    ///
    /// The function should be used to write the value to the device.
    pub fn to_u8(&self) -> u8 {
        let mut value = 0;
        value |= ((self.rf_dr >> 1) << 5) | ((self.rf_dr & 1) << 3);
        if self.cont_wave {
            value |= 0x80;
        }
        value |= (self.rf_pwr & 0b11) << 1;
        value
    }
}

/// The Status Register.
///
/// **rx_dr** - *default: false* - Data Ready RX FIFO interrupt.
///
///   Asserted when new data arrives RX FIFO.
///   Write 1 to clear bit.
///
/// **tx_ds** - *default: false* - Data Sent TX FIFO interrupt.
///   Asserted when packet transmitted on TX. If AUTO_ACK is activated, this bit is set high only when ACK is received.
///   Write 1 to clear bit.
///
/// **max_rt** - *default: false* - Maximum number of TX retransmits interrupt.
///   Maximum number of TX retransmits interrupt.
///   Write 1 to clear bit. If MAX_RT is asserted it must be cleared to be able further transmission.
///
/// **rx_p_no** - *default: 0x07* - Data pipe number for the payload available for reading from RX_FIFO.
///   - 000-101: Data Pipe Number
///   - 110: Not Used
///   - 111: RX FIFO Empty
///
/// **tx_full** - *default: false* - TX FIFO full flag.
///   - true: TX FIFO full.
///   - false: Available locations in TX FIFO.
struct StatusRegister {
    rx_dr: bool,
    tx_ds: bool,
    max_rt: bool,
    rx_p_no: u8,
    tx_full: bool,
}

impl StatusRegister {
    /// Creates a new StatusRegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a StatusRegister.
    pub fn new(value: u8) -> StatusRegister {
        StatusRegister {
            rx_dr: is_bit_set(value, RX_DR),
            tx_ds: is_bit_set(value, TX_DS),
            max_rt: is_bit_set(value, MAX_RT),
            rx_p_no: (value & RX_P_NO) >> 1,
            tx_full: is_bit_set(value, TX_FULL),
        }
    }

    /// Converts the StatusRegister to a u8 value.
    ///
    /// The function should be used to write the value to the device.
    pub fn to_u8(&self) -> u8 {
        let mut value = 0;
        if self.rx_dr {
            value |= 0x40;
        }
        if self.tx_ds {
            value |= 0x20;
        }
        if self.max_rt {
            value |= 0x10;
        }
        value |= (self.rx_p_no & 0b111) << 1;
        if self.tx_full {
            value |= 0x01;
        }
        value
    }
}

/// Transmit observe register.
///
/// **plos_cnt** - *default: 0x00* - Count lost packets.
///   The counter is overflow protected to 15, and discontinues at max until reset.
///   The counter is reset by writing to RF_CH.
///
/// **arc_cnt** - *default: 0x00* - Count retransmitted packets.
///   The counter is reset when transmission of a new packet starts.
struct ObserveTXRegister {
    plos_cnt: u8,
    arc_cnt: u8,
}

impl ObserveTXRegister {
    /// Creates a new ObserveTXRegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a ObserveTXRegister.
    pub fn new(value: u8) -> ObserveTXRegister {
        ObserveTXRegister {
            plos_cnt: (value & PLOS_CNT) >> 4,
            arc_cnt: value & ARC_CNT,
        }
    }
}

/// Received Power Detector.
///
/// **rpd** - *default: false* - Received Power Detector.
///  - true: RPD is above the threshold.
///  - false: RPD is below the threshold.
struct RPDRegister {
    rpd: bool,
}

impl RPDRegister {
    /// Creates a new RPDRegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a RPDRegister.
    pub fn new(value: u8) -> RPDRegister {
        RPDRegister {
            rpd: is_bit_set(value, RPD),
        }
    }
}

struct AddressRegister {
    address: [u8; 5],
    size: u8,
}

impl AddressRegister {
    /// Creates a new AddressRegister from a slice of u8 values.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a RXAddrPRegister.
    fn new(value: &[u8]) -> AddressRegister {
        let size = value.len();
        if size > 5 {
            panic!("Address size must be 5 or less");
        }
        let mut address = [0; 5];
        for i in (0..size).rev() {
            address[i] = value[i];
        }
        AddressRegister {
            address: address,
            size: size as u8,
        }
    }

    /// Converts the AddressRegister to a slice of u8 values.
    ///
    /// The function should be used to write the value to the device.
    fn to_slice(&self) -> &[u8] {
        &self.address
    }
}

/// Number of bytes in RX payload in data pipe.
///
/// **width** - *default: 0x00*
///   0: Pipe not used.
///   1 to 32: Number of bytes in RX payload in data pipe.
struct RXPayloadSizePipeRegister {
    width: u8,
}

impl RXPayloadSizePipeRegister {
    /// Creates a new RXPayloadSizePipeRegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a RXPayloadSizePipeRegister.
    pub fn new(value: u8) -> RXPayloadSizePipeRegister {
        RXPayloadSizePipeRegister { width: value }
    }

    /// Converts the RXPayloadSizePipeRegister to a u8 value.
    ///
    /// The function should be used to write the value to the device.
    pub fn to_u8(&self) -> u8 {
        self.width
    }
}

/// FIFO Status Register.
///
/// **tx_reuse** - *default: false* - Reuse last transmitted payload.
///   Used for a PTX device.
///   Pulse the `rfce` high for at least 10us to Reuse last transmitted payload.
///   TX payload reuse is active until `W_TX_PAYLOAD` or `FLUSH_TX` is executed.
///   TX_REUSE is set by the SPI command `REUSE_TX_PL`, and is reset by the SPI commands `W_TX_PAYLOAD` or `FLUSH_TX`.
///
/// **tx_full** - *default: false* - TX FIFO full flag.
///   - true: TX FIFO full.
///   - false: Available locations in TX FIFO.
///
/// **tx_empty** - *default: true* - TX FIFO empty flag.
///   - true: TX FIFO empty.
///   - false: Data in TX FIFO.
///
/// **rx_full** - *default: false* - RX FIFO full flag.
///   - true: RX FIFO full.
///   - false: Available locations in RX FIFO.
///
/// **rx_empty** - *default: true* - RX FIFO empty flag.
///   - true: RX FIFO empty.
///   - false: Data in RX FIFO.
struct FIFOStatusRegister {
    tx_reuse: bool,
    tx_full: bool,
    tx_empty: bool,
    rx_full: bool,
    rx_empty: bool,
}

impl FIFOStatusRegister {
    /// Creates a new FIFOStatusRegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a FIFOStatusRegister.
    pub fn new(value: u8) -> FIFOStatusRegister {
        FIFOStatusRegister {
            tx_reuse: is_bit_set(value, TX_REUSE),
            tx_full: is_bit_set(value, TX_FULL),
            tx_empty: is_bit_set(value, TX_EMPTY),
            rx_full: is_bit_set(value, RX_FULL),
            rx_empty: is_bit_set(value, RX_EMPTY),
        }
    }
}

/// Dynamic Length Register.
///
/// - **dpl_p5** - *default: false* - Enable dynamic payload length data pipe 5. (Requires `EN_DPL` and `ENAA_P5`)
/// - **dpl_p4** - *default: false* - Enable dynamic payload length data pipe 4. (Requires `EN_DPL` and `ENAA_P4`)
/// - **dpl_p3** - *default: false* - Enable dynamic payload length data pipe 3. (Requires `EN_DPL` and `ENAA_P3`)
/// - **dpl_p2** - *default: false* - Enable dynamic payload length data pipe 2. (Requires `EN_DPL` and `ENAA_P2`)
/// - **dpl_p1** - *default: false* - Enable dynamic payload length data pipe 1. (Requires `EN_DPL` and `ENAA_P1`)
/// - **dpl_p0** - *default: false* - Enable dynamic payload length data pipe 0. (Requires `EN_DPL` and `ENAA_P0`)
struct DynamicPayloadRegister {
    dpl_p5: bool,
    dpl_p4: bool,
    dpl_p3: bool,
    dpl_p2: bool,
    dpl_p1: bool,
    dpl_p0: bool,
}

impl DynamicPayloadRegister {
    /// Creates a new DynamicPayloadRegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a DynamicPayloadRegister.
    pub fn new(value: u8) -> DynamicPayloadRegister {
        DynamicPayloadRegister {
            dpl_p5: is_bit_set(value, DPL_P5),
            dpl_p4: is_bit_set(value, DPL_P4),
            dpl_p3: is_bit_set(value, DPL_P3),
            dpl_p2: is_bit_set(value, DPL_P2),
            dpl_p1: is_bit_set(value, DPL_P1),
            dpl_p0: is_bit_set(value, DPL_P0),
        }
    }

    /// Converts the DynamicPayloadRegister to a u8 value.
    ///
    /// The function should be used to write the value to the device.
    ///
    pub fn to_u8(&self) -> u8 {
        let mut value = 0;
        if self.dpl_p5 {
            value |= DPL_P5;
        }
        if self.dpl_p4 {
            value |= DPL_P4;
        }
        if self.dpl_p3 {
            value |= DPL_P3;
        }
        if self.dpl_p2 {
            value |= DPL_P2;
        }
        if self.dpl_p1 {
            value |= DPL_P1;
        }
        if self.dpl_p0 {
            value |= DPL_P0;
        }
        value
    }
}

/// Feature Register.
///
/// - **en_dpl** - *default: false* - Enables Dynamic Payload Length.
/// - **en_ack_pay** - *default: false* - Enables Payload with ACK.
/// - **en_dyn_ack** - *default: false* - Enables the W_TX_PAYLOAD_NOACK command.
///
/// Note: if ACK packet payload is activated, ACK packets have dynamic payload
/// lengths and the Dynamic Payload Lenght feature must be enabled for pipe 0 on
/// the PTX and PRX. This is to ensure that they receive the ACK packets with
/// payloads. If the ACK payload is more than 15 bytes in 2Mbps mode, the ARD
/// must be 500us or more, and if ACK payload is more than 5 bytes in 1Mbps mode,
/// the ARD must be 500us or more. In 250kbps mode, the ARD must be 500us or more.
struct FeatureRegister {
    en_dpl: bool,
    en_ack_pay: bool,
    en_dyn_ack: bool,
}

impl FeatureRegister {
    /// Creates a new FeatureRegister from a u8 value.
    ///
    /// The function should be used to read the value from the device and
    /// convert it to a FeatureRegister.
    pub fn new(value: u8) -> FeatureRegister {
        FeatureRegister {
            en_dpl: is_bit_set(value, EN_DPL),
            en_ack_pay: is_bit_set(value, EN_ACK_PAY),
            en_dyn_ack: is_bit_set(value, EN_DYN_ACK),
        }
    }

    /// Converts the FeatureRegister to a u8 value.
    ///
    /// The function should be used to write the value to the device.
    ///
    pub fn to_u8(&self) -> u8 {
        let mut value = 0;
        if self.en_dpl {
            value |= EN_DPL;
        }
        if self.en_ack_pay {
            value |= EN_ACK_PAY;
        }
        if self.en_dyn_ack {
            value |= EN_DYN_ACK;
        }
        value
    }
}

struct Registers {
    config: ConfigRegister,
    en_aa: EnableAutoAcknowledgmentRegister,
    en_rxaddr: EnableRXAddressesRegister,
    setup_aw: SetupAddressWidthsRegister,
    setup_retr: SetupRetransmitsRegister,
    rf_ch: RFChannelRegister,
    rf_setup: RFSetupRegister,
    status: StatusRegister,
    observe_tx: ObserveTXRegister,
    rpd: RPDRegister,
    rx_addr_p0: AddressRegister,
    rx_addr_p1: AddressRegister,
    rx_addr_p2: AddressRegister,
    rx_addr_p3: AddressRegister,
    rx_addr_p4: AddressRegister,
    rx_addr_p5: AddressRegister,
    tx_addr: AddressRegister,
    rx_pw_p0: RXPayloadSizePipeRegister,
    rx_pw_p1: RXPayloadSizePipeRegister,
    rx_pw_p2: RXPayloadSizePipeRegister,
    rx_pw_p3: RXPayloadSizePipeRegister,
    rx_pw_p4: RXPayloadSizePipeRegister,
    rx_pw_p5: RXPayloadSizePipeRegister,
    fifo_status: FIFOStatusRegister,
    dynpd: DynamicPayloadRegister,
    feature: FeatureRegister,
}

struct RF24 {
    registers: Registers,
    ce_pin: OutputPin,
    spi: Spi,
}

impl RF24 {
    /// Creates a new RF24 instance.
    ///
    /// The function takes the SPI interface and the CE pin as arguments.
    ///
    /// The function returns a new RF24 instance.
    pub fn new(ce_pin: OutputPin) -> Result<Self, RF24Error> {
        let ce_pin = Gpio::new()
            .map_err(|e| RF24Error::GpioError(e))?
            .get(ce_pin)
            .map_err(|e| RF24Error::GpioError(e))?
            .into_output_low();

        let spi: Spi = Spi::new(
            spi::Bus::Spi0,
            spi::SlaveSelect::Ss0,
            SPI_SPEED_MHZ,
            spi::Mode::Mode0,
        )
        .map_err(|e| RF24Error::SpiError(e))?;

        sleep(Duration::from_millis(5));

        Ok(RF24 {
            registers: Registers {
                config: ConfigRegister::new(0x0E),
                en_aa: EnableAutoAcknowledgmentRegister::new(0x3F),
                en_rxaddr: EnableRXAddressesRegister::new(0x03),
                setup_aw: SetupAddressWidthsRegister::new(0x03),
                setup_retr: SetupRetransmitsRegister::new(0x03),
                rf_ch: RFChannelRegister::new(0x02),
                rf_setup: RFSetupRegister::new(0x0E),
                status: StatusRegister::new(0x0E),
                observe_tx: ObserveTXRegister::new(0),
                rpd: RPDRegister::new(0),
                rx_addr_p0: AddressRegister::new(&[0xE7, 0xE7, 0xE7, 0xE7, 0xE7]),
                rx_addr_p1: AddressRegister::new(&[0xC2, 0xC2, 0xC2, 0xC2, 0xC2]),
                rx_addr_p2: AddressRegister::new(&[0xC3]),
                rx_addr_p3: AddressRegister::new(&[0xC4]),
                rx_addr_p4: AddressRegister::new(&[0xC5]),
                rx_addr_p5: AddressRegister::new(&[0xC6]),
                tx_addr: AddressRegister::new(&[0xE7, 0xE7, 0xE7, 0xE7, 0xE7]),
                rx_pw_p0: RXPayloadSizePipeRegister::new(0),
                rx_pw_p1: RXPayloadSizePipeRegister::new(0),
                rx_pw_p2: RXPayloadSizePipeRegister::new(0),
                rx_pw_p3: RXPayloadSizePipeRegister::new(0),
                rx_pw_p4: RXPayloadSizePipeRegister::new(0),
                rx_pw_p5: RXPayloadSizePipeRegister::new(0),
                fifo_status: FIFOStatusRegister::new(0x11),
                dynpd: DynamicPayloadRegister::new(0),
                feature: FeatureRegister::new(0),
            },
            ce_pin,
            spi,
        })
    }
}
