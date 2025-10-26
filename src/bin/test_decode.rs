extern crate core;

use apator_wmbus_mqtt::wmbus::wmbus_decode::decode_packet;

fn main() {
    let hex_string = "6e44010647647301050749d57a34006085a1a148562c1c483c1775487340dfc45ad68bb176e846c9e4982d9e345d0019a98ad6e6b8bd3cca9c618f56faac2712ddc7dd9bac3198ce9ebc666d46e62d74cd91b9bb1ba6e4f89ecf47f6d4bce8a635c259a87c30dc80b17c552c473b847ba5d83cb261df5b4eeea9b878987745";
    let bytes = hex::decode(hex_string).expect("Invalid hex string");

    let telegram = decode_packet(&bytes);

    println!("telegram: {:?}", telegram.unwrap().total_water_volume);
}
