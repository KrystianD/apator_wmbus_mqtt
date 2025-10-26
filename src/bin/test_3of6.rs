use apator_wmbus_mqtt::utils::bytes_to_hex;
use apator_wmbus_mqtt::wmbus::decode_3of6::decode_3of6;
use apator_wmbus_mqtt::wmbus::wmbus_crc::is_valid_crc;
use binary_reader::{BinaryReader, Endian};
use std::string::ToString;

//noinspection SpellCheckingInspection
fn main() {
    let hex_string: String = "6b271c58d59a71369c4cb58d599593725c594e62dc596696b1998d98d72c65a3b437472c2f43534d972c4cb716c69d1c666c5ab238cd4dacac71ad25c9c96c3b19722dc6715963659a5b26c5ac9a8ec8f12f4d2697468db2965aa669b439334ec71d13c719639b42cd96cd329728f469a6b171ac9a3b14dcd3194d8e58e336399ac9ca6c972d29713a5ac5c8f4cac99a2d9d0e6659ac4f42d6c74b168cd4f46593b47132e3b1c4e3999c6c2f48ce68dc69663732cb29a58ec4ec96c4d371955feaefad3ecaba57e7bdedbfefedcff7dfd6fffff6b36bace8d57fae7fbfed1f3fb7fdf5ff57fbfffcdfd7f7effe7bbfcf6dffdcffe7bbefe5fffbbd7f7e7f7d".to_string();
    let bytes = hex::decode(hex_string).expect("Invalid hex string");

    let decoded = decode_3of6(&bytes);
    println!("decoded: {}", bytes_to_hex(&decoded));

    let mut reader = BinaryReader::from_vec(&decoded);
    reader.set_endian(Endian::Big);

    let header_bytes = reader.read_bytes(10).unwrap().to_vec();
    let header_crc = reader.read_u16().unwrap();

    let ok = is_valid_crc(header_bytes.as_slice(), header_crc);
    println!("crc ok: {}", ok);
}
