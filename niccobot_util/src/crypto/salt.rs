use rand::{rngs::OsRng, RngCore};

pub fn generate_secure_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    salt
}