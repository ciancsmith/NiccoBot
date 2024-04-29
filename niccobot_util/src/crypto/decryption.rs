use std::error::Error;
use aes_gcm::aead::{Aead};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::generic_array::GenericArray;
use rand::{rngs::OsRng, RngCore};
use argon2::Argon2;


pub fn decrypt(ciphertext: &[u8], nonce_bytes: &[u8; 12], key_bytes: &[u8; 32]) -> Result<String, anyhow::Error> {
    let key = GenericArray::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted_data = cipher.decrypt(nonce, ciphertext).map_err(anyhow::Error::msg)?;

    // Assuming the decrypted data is valid UTF-8, convert it to a String
    let plaintext = String::from_utf8(decrypted_data).map_err(anyhow::Error::msg)?;
    Ok(plaintext)
}