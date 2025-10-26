#[derive(Debug)]
pub struct Telegram {
    pub l_field: u8,
    pub last_block_length: u8,
    pub dll_mfct: Vec<u8>,
    pub a_field: Vec<u8>,
    pub device_identifier: String,

    pub app_payload: Vec<u8>,

    pub ci_field: u8,
    pub tpl_acc: u8,
    pub sts_field: u8,
    pub cfg_field: u16,

    pub encrypted: Vec<u8>,
    pub payload_decrypted: Vec<u8>,

    pub total_water_volume: u32,
}
