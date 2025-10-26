use apator_wmbus_mqtt::wmbus::decrypter::decrypt_nullkey;

fn main() {
    let encrypted_hex = "20e62df5fd1c060b40f1811becdd8b71d41b2c8ddb9d30069bd67577322f523c89925dc2c7135a770fa23ed1d93ea8fc286cee088f4c8ff497820e5af782fb2827080bc9a4df73d7a6f0b4b97b3003565b749bde4032a658adb8527e432fcbde";
    let encrypted = hex::decode(encrypted_hex).expect("Invalid hex string");

    let plaintext = decrypt_nullkey(&[0; 16], &encrypted);

    println!("{plaintext:?}");
}
