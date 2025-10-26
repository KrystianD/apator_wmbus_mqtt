use crc::{Crc, CRC_16_EN_13757};

const CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_EN_13757);

pub fn is_valid_crc(block: &[u8], expected: u16) -> bool {
    let index = block.len();

    let mut digest = CRC.digest();
    digest.update(&block[0..index]);
    let actual = digest.finalize();

    actual == expected
}
