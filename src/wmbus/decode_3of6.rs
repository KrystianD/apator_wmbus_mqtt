use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::string::ToString;

static DECODE_3_OUT_OF_6: Lazy<HashMap<&'static str, u8>> = Lazy::new(|| {
    HashMap::from([
        ("001011", 3),
        ("001101", 1),
        ("001110", 2),
        ("010011", 7),
        ("010110", 0),
        ("011001", 5),
        ("011010", 6),
        ("011100", 4),
        ("100011", 11),
        ("100101", 9),
        ("100110", 10),
        ("101001", 15),
        ("101100", 8),
        ("110001", 13),
        ("110010", 14),
        ("110100", 12),
    ])
});

const END_MARK: &str = &"010101";
const CMODE: &str = &"01010100";

pub fn decode_3of6(bytes: &Vec<u8>) -> Vec<u8> {
    if bytes[0] == 0x54 {
        // CMODE
        return vec![];
    }

    // convert bytes into bitstream,
    // split into 6-bit chunks and decode each chunk into a 4-bit value using the 3-of-6 lookup table,
    // then join the resulting 8-bit values into a single byte.

    bytes
        .iter()
        .map(|x| format!("{:08b}", x))
        .collect::<Vec<String>>()
        .join("")
        .chars()
        .collect::<Vec<char>>()
        .chunks(6)
        .map(|x| x.iter().collect::<String>())
        .take_while(|x| x != END_MARK)
        .map(|x| {
            DECODE_3_OUT_OF_6
                .get(x.as_str())
                .unwrap_or(&0xffu8)
        })
        .map(|x| format!("{:04b}", x).to_string())
        .collect::<Vec<String>>()
        .join("")
        .chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|x| x.iter().collect::<String>())
        .map(|x| u8::from_str_radix(x.as_str(), 2).unwrap_or(0xffu8))
        .collect()
}
