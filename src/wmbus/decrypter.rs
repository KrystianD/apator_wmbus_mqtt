use cbc::cipher::BlockDecryptMut;
use cbc::cipher::block_padding::NoPadding;
use cbc::cipher::KeyIvInit;
use anyhow::{Result, anyhow};

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

pub fn decrypt_nullkey(iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
    let key = [0; 16];
    let decryptor = Aes128CbcDec::new(&key.into(), iv.into());
    let plaintext = decryptor.decrypt_padded_vec_mut::<NoPadding>(ciphertext)
                             .map_err(|e| anyhow!("Decryption error: {}", e))?;

    Ok(plaintext)
}
