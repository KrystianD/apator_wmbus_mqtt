use spidev::{Spidev, SpidevOptions, SpidevTransfer};

pub struct SpiDevice {
    spi: Spidev,
}

impl SpiDevice {
    pub fn create(path: &str, rate: u32) -> Self {
        let mut spi = Spidev::open(path).unwrap();

        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(rate)
            .mode(spidev::SpiModeFlags::SPI_MODE_0)
            .build();

        spi.configure(&options).unwrap();

        Self { spi }
    }

    pub fn xfer(&self, buf: &[u8]) -> Vec<u8> {
        let mut recv_data: Vec<u8> = Vec::new();
        recv_data.resize(buf.len(), 0);

        let mut transfer = SpidevTransfer::read_write(&buf, &mut recv_data);
        self.spi.transfer(&mut transfer).unwrap();

        recv_data
    }
}
