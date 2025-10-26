use crate::cc1101::cc1101_config::CC1101Config;
use crate::cc1101::cc1101_enums::{CommandStrobe, ConfigurationRegisterAddress, StatusRegisterAddress};
use crate::spi_device::SpiDevice;
use num_traits::FromPrimitive;
use std::fmt::Debug;
use crate::tprintln;

pub struct CC1101 {
    spi: SpiDevice,
}

impl CC1101 {}

impl CC1101 {
    pub fn new(spi: SpiDevice) -> Self {
        Self { spi }
    }

    pub fn write_registers_burst(&self, reg: u8, val: &[u8]) {
        self.spi
            .xfer(&[[0x40 + reg].to_vec(), val.to_vec()].concat());
    }

    pub fn read_registers_burst(&self, reg: u8, count: u8) -> Vec<u8> {
        self.spi
            .xfer(&[[0xC0 + reg].to_vec(), vec![0xff; count as usize]].concat())[1..]
            .to_vec()
    }

    // API
    pub fn read_config(&self, reg: ConfigurationRegisterAddress) -> u8 {
        self.read_registers_burst(reg as u8, 1)[0]
    }

    pub fn write_config<T>(&self, reg: ConfigurationRegisterAddress, val: T)
    where
        T: Into<u8> + Debug + Copy,
    {
        let val_u8 = val.into();

        tprintln!("set 0x{:02x} to 0x{:02x} - {:?}", reg as u8, val_u8, val);
        self.write_registers_burst(reg as u8, &[val_u8]);

        let read_back = self.read_config(reg);

        if read_back != val_u8 {
            panic!("write failed");
        }
    }

    pub fn read_status(&self, reg: StatusRegisterAddress) -> u8 {
        self.read_registers_burst(reg as u8, 1)[0] // must use burst read
    }

    pub fn command(&self, cmd: CommandStrobe) {
        self.spi.xfer(&[cmd as u8]);
    }

    //
    pub fn apply_config(&self, config: CC1101Config) {
        config.bytes.iter().for_each(|(reg, val)| {
            self.write_config(ConfigurationRegisterAddress::from_u8(*reg).unwrap(), *val);
        })
    }
}
