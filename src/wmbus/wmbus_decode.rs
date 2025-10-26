use crate::wmbus::decrypter::decrypt_nullkey;
use crate::wmbus::telegram::Telegram;
use crate::wmbus::wmbus_crc::is_valid_crc;
use anyhow::{anyhow, Context};
use binary_reader::{BinaryReader, Endian};

fn register_size(c: u8) -> i32 {
    match c {
        // case 0x00
        0x00 => 4, // Date
        0x01 => 3, // Faults

        0xA1 | 0x10 => 4, // Total volume
        0x11 => 2,        // Flow

        0x40 => 6, // Detectors
        0x41 => 2, // Voltage
        0x42 => 4, // Energy
        0x43 => 2, // Life days
        0x44 => 3,

        0x71 => 1 + 2 * 4,
        0x72 => 1 + 3 * 4,
        0x73 => 1 + 4 * 4,
        0x74 => 1 + 5 * 4,
        0x75 => 1 + 6 * 4,
        0x76 => 1 + 7 * 4,
        0x77 => 1 + 8 * 4,
        0x78 => 1 + 9 * 4,
        0x79 => 1 + 10 * 4,
        0x7A => 1 + 11 * 4,
        0x7B => 1 + 12 * 4,

        0x80 | 0x81 | 0x82 | 0x83 | 0x84 | 0x86 | 0x87 => 10, // Events
        0x85 | 0x88 | 0x8F => 11,                             // Events
        0x8A => 9,                                            // Events
        0x8B | 0x8C => 6,                                     // Events
        0x8E => 7,                                            // Events

        0xA0 => 4,
        0xA2 => 1,
        0xA3 => 7,
        0xA4 => 4,
        0xA5 | 0xA9 | 0xAF => 1,
        0xA6 => 3,
        0xA7 | 0xA8 | 0xAA | 0xAB | 0xAC | 0xAD => 2,

        0xB0 => 5,
        0xB1 => 8,
        0xB2 => 16,
        0xB3 => 8,
        0xB4 => 2,
        0xB5 => 16,

        0xB6..=0xBF => 3, // Unknown
        0xC0..=0xC7 => 3,
        0xD0 | 0xD3 => 3,
        0xF0 => 4,

        _ => -1,
    }
}

pub fn decode_packet(data: &[u8]) -> Result<Telegram, anyhow::Error> {
    let mut packet_reader = BinaryReader::from_u8(data);
    packet_reader.set_endian(Endian::Big);

    let mut telegram = Telegram {
        l_field: 0,
        last_block_length: 0,
        dll_mfct: Vec::new(),
        a_field: Vec::new(),
        device_identifier: "".into(),
        app_payload: Vec::new(),
        ci_field: 0,
        tpl_acc: 0,
        sts_field: 0,
        cfg_field: 0,
        encrypted: Vec::new(),
        payload_decrypted: Vec::new(),
        total_water_volume: 0,
    };

    let header_bytes;
    {
        header_bytes = packet_reader.read_bytes(10)?.to_vec();
        let header_crc = packet_reader.read_u16()?;
        let ok = is_valid_crc(header_bytes.as_slice(), header_crc);
        if !ok {
            return Err(anyhow!("invalid crc"));
        }
    }

    {
        let mut header_reader = BinaryReader::from_vec(&header_bytes);
        header_reader.set_endian(Endian::Big);

        telegram.l_field = header_reader.read_u8()?;
        telegram.last_block_length = (telegram.l_field - 9) % 16;
        header_reader.read_u8()?;
        telegram.dll_mfct = header_reader.read_bytes(2)?.to_vec();
        telegram.a_field = header_reader.read_bytes(6)?.to_vec();
        telegram.device_identifier = telegram.a_field[0..4]
            .iter()
            .rev()
            .map(|x| format!("{:02x}", x))
            .collect::<String>();
    }

    {
        while packet_reader.pos < packet_reader.length {
            let remaining = packet_reader.length - packet_reader.pos;
            let block_length = if remaining < 16 { telegram.last_block_length } else { 16 };
            let block_bytes = packet_reader.read_bytes(block_length as usize)?.to_vec();

            let block_crc = packet_reader.read_u16()?;
            let ok = is_valid_crc(&block_bytes, block_crc);
            if !ok {
                return Err(anyhow!("invalid crc"));
            }

            telegram.app_payload.extend(block_bytes);
        }
    }

    {
        let mut payload_reader = BinaryReader::from_vec(&telegram.app_payload);
        payload_reader.set_endian(Endian::Big);

        telegram.ci_field = payload_reader.read_u8()?;
        if telegram.ci_field != 0x7a {
            return Err(anyhow!("invalid ci_field"));
        }

        telegram.tpl_acc = payload_reader.read_u8()?;

        telegram.sts_field = payload_reader.read_u8()?;
        if telegram.sts_field != 0x00 {
            return Err(anyhow!("invalid sts_field"));
        }

        telegram.cfg_field = payload_reader.read_u16()?;
        if telegram.cfg_field != 0x6085 {
            return Err(anyhow!("invalid cfg_field"));
        }

        telegram.encrypted = payload_reader.read_bytes(payload_reader.length - payload_reader.pos)?.to_vec();
    }

    {
        let iv = [telegram.dll_mfct.clone(), telegram.a_field.clone(), [telegram.tpl_acc; 8].to_vec()].concat();

        telegram.payload_decrypted = decrypt_nullkey(&iv, &telegram.encrypted)?;
        if telegram.payload_decrypted[0..2] != [0x2f, 0x2f] {
            return Err(anyhow!("invalid payload"));
        }
    }

    let mut total_water_volume = None;

    {
        let mut apator_payload_reader = BinaryReader::from_vec(&telegram.payload_decrypted);
        apator_payload_reader.set_endian(Endian::Little);

        apator_payload_reader.read_bytes(2)?;
        apator_payload_reader.read_bytes(8)?;

        while apator_payload_reader.pos < apator_payload_reader.length {
            let register_type = apator_payload_reader.read_u8()?;
            if register_type == 0xff {
                break;
            }

            let register_size = register_size(register_type);
            if register_size == -1 {
                return Err(anyhow!("invalid register_type"));
            }

            let register_value = apator_payload_reader.read_bytes(register_size as usize)?.to_vec();

            // tprintln!(
            //     "type: {:02x}, size: {}, value: {:?}",
            //     register_type,
            //     register_size,
            //     bytes_to_hex(&register_value)
            // );

            if register_type == 0x10 {
                // parse uint32 from register_value using some helper
                let total = u32::from_le_bytes(register_value.as_slice().try_into()?);

                total_water_volume = Some(total);
            }
        }
    }

    telegram.total_water_volume = total_water_volume.context("total_water_volume not found")?;

    Ok(telegram)
}
