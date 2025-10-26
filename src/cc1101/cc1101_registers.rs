#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use binrw::binrw;
use modular_bitfield::prelude::*;

pub const FXOSC: u64 = 26_000_000;

#[derive(Specifier)]
#[bits = 6]
#[derive(Debug)]
pub enum GdoCfg {
    RX_FIFO_FILLED = 0x00,
    RX_FIFO_FILLED_END_OF_PKT = 0x01,
    TX_FIFO_FILLED = 0x02,
    TX_FIFO_FULL = 0x03,
    RX_FIFO_OVERFLOW = 0x04,
    TX_FIFO_UNDERFLOW = 0x05,
    SYNC_WORD = 0x06,
    CRC_OK = 0x07,
    PQT_REACHED = 0x08,
    CHANNEL_CLEAR = 0x09,
    PLL_LOCK = 0x0A,
    SERIAL_CLOCK = 0x0B,
    SERIAL_SYNC_DATA_OUT = 0x0C,
    SERIAL_DATA_OUT = 0x0D,
    CARRIER_SENSE = 0x0E,
    LAST_CRC_OK = 0x0F,

    RX_HARD_DATA_1 = 0x16,
    RX_HARD_DATA_0 = 0x17,

    PA_PD = 0x1B,
    LNA_PD = 0x1C,
    RX_SYMBOL_TICK = 0x1D,

    WOR_EVNT0 = 0x24,
    WOR_EVNT1 = 0x25,
    CLK_256 = 0x26,
    CLK_32k = 0x27,

    CHIP_RDYn = 0x29,

    XOSC_STABLE = 0x2B,

    HIGH_IMPEDANCE = 0x2E,
    HARDWIRE_TO_0 = 0x2F,
    CLK_XOSC_1 = 0x30,
    CLK_XOSC_1_5 = 0x31,
    CLK_XOSC_2 = 0x32,
    CLK_XOSC_3 = 0x33,
    CLK_XOSC_4 = 0x34,
    CLK_XOSC_6 = 0x35,
    CLK_XOSC_8 = 0x36,
    CLK_XOSC_12 = 0x37,
    CLK_XOSC_16 = 0x38,
    CLK_XOSC_24 = 0x39,
    CLK_XOSC_32 = 0x3A,
    CLK_XOSC_48 = 0x3B,
    CLK_XOSC_64 = 0x3C,
    CLK_XOSC_96 = 0x3D,
    CLK_XOSC_128 = 0x3E,
    CLK_XOSC_192 = 0x3F,
}

impl From<GdoCfg> for u8 {
    fn from(value: GdoCfg) -> Self {
        value as Self
    }
}

macro_rules! impl_from_int {
    ($ty:ty) => {
        impl $ty {
            pub fn from(val: u8) -> Self {
                Self::from_bytes([val])
            }
        }

        impl From<$ty> for u8 {
            fn from(value: $ty) -> Self {
                <u8>::from_le_bytes(value.into_bytes())
            }
        }
    };
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct IOCFG2 {
    pub GDO2_CFG: GdoCfg,
    pub GDO2_INV: B1,
    #[skip]
    __: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct IOCFG1 {
    pub GDO1_CFG: GdoCfg,
    pub GDO1_INV: B1,
    pub GDO_DS: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct IOCFG0 {
    pub GDO0_CFG: GdoCfg,
    pub GDO0_INV: B1,
    pub TEMP_SENSOR_ENABLE: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FIFOTHR {
    pub FIFO_THR: B4,
    pub CLOSE_IN_RX: B2,
    pub ADC_RETENTION: B1,
    #[skip]
    __: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct SYNC1 {
    pub SYNC: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct SYNC0 {
    pub SYNC: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PKTLEN {
    pub PACKET_LENGTH: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PKTCTRL1 {
    pub ADR_CHK: B2,
    pub APPEND_STATUS: B1,
    pub CRC_AUTOFLUSH: B1,
    #[skip]
    __: B1,
    pub PQT: B3,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PKTCTRL0 {
    pub LENGTH_CONFIG: B2,
    pub CRC_EN: B1,
    #[skip]
    __: B1,
    pub PKT_FORMAT: B2,
    pub WHITE_DATA: B1,
    #[skip]
    __: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct ADDR {
    pub DEVICE_ADDR: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct CHANNR {
    pub CHAN: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FSCTRL1 {
    pub FREQ_IF: B5,
    #[skip]
    __: B3,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FSCTRL0 {
    pub FREQOFF: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FREQ2 {
    pub FREQ: B6,
    #[skip]
    __: B2,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FREQ1 {
    pub FREQ: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FREQ0 {
    pub FREQ: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct MDMCFG4 {
    pub DRATE_E: B4,
    pub CHANBW_M: B2,
    pub CHANBW_E: B2,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct MDMCFG3 {
    pub DRATE_M: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct MDMCFG2 {
    pub SYNC_MODE: B3,
    pub MANCHESTER_EN: B1,
    pub MOD_FORMAT: B3,
    pub DEM_DCFILT_OFF: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct MDMCFG1 {
    pub CHANSPC_E: B2,
    #[skip]
    __: B2,
    pub NUM_PREAMBLE: B3,
    pub FEC_EN: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct MDMCFG0 {
    pub CHANSPC_M: B8,
}

#[bitfield(bits = 8)]
// #[repr(u8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct DEVIATN {
    pub DEVIATION_M: B3,
    #[skip]
    __: B1,
    pub DEVIATION_E: B3,
    #[skip]
    __: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct MCSM2 {
    pub RX_TIME: B3,
    pub RX_TIME_QUAL: B1,
    pub RX_TIME_RSSI: B1,
    #[skip]
    __: B3,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct MCSM1 {
    pub TXOFF_MODE: B2,
    pub RXOFF_MODE: B2,
    pub CCA_MODE: B2,
    #[skip]
    __: B2,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct MCSM0 {
    pub XOSC_FORCE_ON: B1,
    pub PIN_CTRL_EN: B1,
    pub PO_TIMEOUT: B2,
    pub FS_AUTOCAL: B2,
    #[skip]
    __: B2,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FOCCFG {
    pub FOC_LIMIT: B2,
    pub FOC_POST_K: B1,
    pub FOC_PRE_K: B2,
    pub FOC_BS_CS_GATE: B1,
    #[skip]
    __: B2,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct BSCFG {
    pub BS_LIMIT: B2,
    pub BS_POST_KP: B1,
    pub BS_POST_KI: B1,
    pub BS_PRE_KP: B2,
    pub BS_PRE_KI: B2,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct AGCCTRL2 {
    pub MAGN_TARGET: B3,
    pub MAX_LNA_GAIN: B3,
    pub MAX_DVGA_GAIN: B2,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct AGCCTRL1 {
    pub CARRIER_SENSE_ABS_THR: B4,
    pub CARRIER_SENSE_REL_THR: B2,
    pub AGC_LNA_PRIORITY: B1,
    #[skip]
    __: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct AGCCTRL0 {
    pub FILTER_LENGTH: B2,
    pub AGC_FREEZE: B2,
    pub WAIT_TIME: B2,
    pub HYST_LEVEL: B2,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct WOREVT1 {
    pub EVENT0: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct WOREVT0 {
    pub EVENT0: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct WORCTRL {
    pub WOR_RES: B2,
    #[skip]
    __: B1,
    pub RC_CAL: B1,
    pub EVENT1: B3,
    pub RC_PD: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FREND1 {
    pub MIX_CURRENT: B2,
    pub LODIV_BUF_CURRENT_RX: B2,
    pub LNA2MIX_CURRENT: B2,
    pub LNA_CURRENT: B2,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FREND0 {
    pub PA_POWER: B3,
    #[skip]
    __: B1,
    pub LODIV_BUF_CURRENT_T: B2,
    #[skip]
    __: B2,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FSCAL3 {
    pub FSCAL3_2: B4,
    pub CHP_CURR_CAL_EN: B2,
    pub FSCAL3_1: B2,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FSCAL2 {
    pub FSCAL2: B5,
    pub VCO_CORE_H_EN: B1,
    #[skip]
    __: B2,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FSCAL1 {
    pub FSCAL1: B6,
    #[skip]
    __: B2,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FSCAL0 {
    pub FSCAL0: B7,
    #[skip]
    __: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct RCCTRL1 {
    pub RCCTRL1: B7,
    #[skip]
    __: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct RCCTRL0 {
    pub RCCTRL0: B7,
    #[skip]
    __: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FSTEST {
    pub FSTEST: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PTEST {
    pub PTEST: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct AGCTEST {
    pub AGCTEST: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TEST2 {
    pub TEST2: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TEST1 {
    pub TEST1: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TEST0 {
    pub TEST0_2: B1,
    pub VCO_SEL_CAL_EN: B1,
    pub TEST0_1: B6,
}

// Status
#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PARTNU {
    pub PARTNUM: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct VERSION {
    pub VERSION: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FREQEST {
    pub FREQOFF_EST: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct LQI {
    pub LQI_EST: B7,
    pub CRC_OK: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct RSSI {
    pub RSSI: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct MARCSTATE {
    pub MARC_STATE: B5,
    #[skip]
    __: B3,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct WORTIME1 {
    pub TIME: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct WORTIME0 {
    pub TIME: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PKTSTATUS {
    pub GDO0: B1,
    #[skip]
    __: B1,
    pub GDO2: B1,
    pub SFD: B1,
    pub CCA: B1,
    pub PQT_REACHED: B1,
    pub CS: B1,
    pub CRC_OK: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct VCO_VC_DAC {
    pub VCO_VC_DAC: B8,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct TXBYTES {
    pub NUM_TXBYTES: B7,
    pub TXFIFO_UNDERFLOW: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct RXBYTES {
    pub NUM_RXBYTES: B7,
    pub RXFIFO_OVERFLOW: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct RCCTRL1_STATUS {
    pub RCCTRL1_STATUS: B7,
    #[skip]
    __: B1,
}

#[bitfield(bits = 8)]
#[derive(Debug, Default, Clone, Copy)]
pub struct RCCTRL0_STATUS {
    pub RCCTRL0_STATUS: B7,
    #[skip]
    __: B1,
}

impl_from_int!(IOCFG2);
impl_from_int!(IOCFG1);
impl_from_int!(IOCFG0);
impl_from_int!(FIFOTHR);
impl_from_int!(SYNC1);
impl_from_int!(SYNC0);
impl_from_int!(PKTLEN);
impl_from_int!(PKTCTRL1);
impl_from_int!(PKTCTRL0);
impl_from_int!(ADDR);
impl_from_int!(CHANNR);
impl_from_int!(FSCTRL1);
impl_from_int!(FSCTRL0);
impl_from_int!(FREQ2);
impl_from_int!(FREQ1);
impl_from_int!(FREQ0);
impl_from_int!(MDMCFG4);
impl_from_int!(MDMCFG3);
impl_from_int!(MDMCFG2);
impl_from_int!(MDMCFG1);
impl_from_int!(MDMCFG0);
impl_from_int!(DEVIATN);
impl_from_int!(MCSM2);
impl_from_int!(MCSM1);
impl_from_int!(MCSM0);
impl_from_int!(FOCCFG);
impl_from_int!(BSCFG);
impl_from_int!(AGCCTRL2);
impl_from_int!(AGCCTRL1);
impl_from_int!(AGCCTRL0);
impl_from_int!(WOREVT1);
impl_from_int!(WOREVT0);
impl_from_int!(WORCTRL);
impl_from_int!(FREND1);
impl_from_int!(FREND0);
impl_from_int!(FSCAL3);
impl_from_int!(FSCAL2);
impl_from_int!(FSCAL1);
impl_from_int!(FSCAL0);
impl_from_int!(RCCTRL1);
impl_from_int!(RCCTRL0);
impl_from_int!(FSTEST);
impl_from_int!(PTEST);
impl_from_int!(AGCTEST);
impl_from_int!(TEST2);
impl_from_int!(TEST1);
impl_from_int!(TEST0);
impl_from_int!(PARTNU);
impl_from_int!(VERSION);
impl_from_int!(FREQEST);
impl_from_int!(LQI);
impl_from_int!(RSSI);
impl_from_int!(MARCSTATE);
impl_from_int!(WORTIME1);
impl_from_int!(WORTIME0);
impl_from_int!(PKTSTATUS);
impl_from_int!(VCO_VC_DAC);
impl_from_int!(TXBYTES);
impl_from_int!(RXBYTES);
impl_from_int!(RCCTRL1_STATUS);
impl_from_int!(RCCTRL0_STATUS);
