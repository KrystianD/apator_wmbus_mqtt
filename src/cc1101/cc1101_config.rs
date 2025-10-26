use crate::cc1101::cc1101_enums::ConfigurationRegisterAddress;
use crate::cc1101::cc1101_registers::{DEVIATN, FREQ0, FREQ1, FREQ2, FXOSC, SYNC0, SYNC1, FSCTRL1};
use crate::tprintln;
use std::collections::HashMap;
use std::fmt::Debug;

pub struct CC1101Config {
    pub bytes: HashMap<u8, u8>,
}

impl CC1101Config {
    pub fn new() -> Self {
        CC1101Config { bytes: HashMap::new() }
    }

    pub fn read_config<T>(&self, reg: ConfigurationRegisterAddress) -> T
    where
        T: From<u8>,
    {
        let r = reg as u8;
        let q = self.bytes.get(&r);

        match q {
            Some(v) => (*v).into(),
            None => panic!("reg not found: {:?}", reg),
        }
    }

    pub fn write_config<T>(&mut self, reg: ConfigurationRegisterAddress, val: T)
    where
        T: Into<u8> + Debug + Copy,
    {
        let val_u8 = val.into();

        tprintln!("set 0x{:02x} to 0x{:02x} - {:?}", reg as u8, val_u8, val);

        self.bytes.insert(reg as u8, val_u8);
    }

    #[allow(non_snake_case)]
    pub fn write_FREQ(&mut self, freq: u64) {
        let freq = freq * u64::pow(2, 16) / FXOSC;

        let freq2 = ((freq >> 16) & 0x3f) as u8;
        let freq1 = ((freq >> 8) & 0xff) as u8;
        let freq0 = ((freq >> 0) & 0xff) as u8;

        self.write_config(ConfigurationRegisterAddress::FREQ2, FREQ2::from(freq2));
        self.write_config(ConfigurationRegisterAddress::FREQ1, FREQ1::from(freq1));
        self.write_config(ConfigurationRegisterAddress::FREQ0, FREQ0::from(freq0));
    }

    #[allow(non_snake_case)]
    pub fn write_SYNC(&mut self, sync_word: u16) {
        self.write_config(ConfigurationRegisterAddress::SYNC1, SYNC1::from(((sync_word >> 8) & 0xff) as u8));
        self.write_config(ConfigurationRegisterAddress::SYNC0, SYNC0::from(((sync_word >> 0) & 0xff) as u8));
    }

    #[allow(non_snake_case)]
    pub fn write_DEVIATN(&mut self, deviation_freq: u64) {
        let mut best: (u8, u8) = (0, 0);
        let mut best_error: u64 = 0xffffffffffffffff;

        for exponent in 0..=7 {
            for mantissa in 0..=7 {
                let int = (2u64.pow(exponent as u32) * (8 + mantissa as u64) * FXOSC / 2u64.pow(17)) as i64;
                let error = (deviation_freq as i64 - int).abs() as u64;
                if error < best_error {
                    best = (exponent, mantissa);
                    best_error = error;
                }
            }
        }

        self.write_config(
            ConfigurationRegisterAddress::DEVIATN,
            DEVIATN::new()
                .with_DEVIATION_E(best.0)
                .with_DEVIATION_M(best.1),
        );
    }

    #[allow(non_snake_case)]
    pub fn write_FREQ_IF(&mut self, freq_if: u32) {
        assert!(freq_if <= 787109);

        let div = ((freq_if as u64 * 2u64.pow(10)) as f64 / FXOSC as f64).round() as u8;

        self.write_config(ConfigurationRegisterAddress::FSCTRL1, FSCTRL1::new().with_FREQ_IF(div));
    }

    pub fn print(&self) {
        self.bytes.iter().for_each(|(reg, val)| {
            tprintln!("set 0x{:02x} to 0x{:02x}", *reg, *val);
        })
    }
}
