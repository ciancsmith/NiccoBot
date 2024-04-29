use aes_gcm::aead::{Aead};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::generic_array::GenericArray;
use rand::{rngs::OsRng, RngCore};
use argon2::Argon2;


pub fn encrypt(password: &str, key_bytes: &[u8; 32]) ->Result<(Vec<u8>, [u8; 12]), anyhow::Error> {
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = GenericArray::from_slice(&nonce_bytes);
    if key_bytes.len() != 32 {
        panic!("Key must be 256 bits (32 bytes) long: yours is {}", key_bytes.len());
    }

    let key =  GenericArray::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(&key);

    let ciphertext = cipher.encrypt(nonce, password.as_bytes()).map_err(anyhow::Error::msg)?;
    Ok((ciphertext, nonce_bytes))

}