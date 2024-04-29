use argon2::{Argon2, Error, PasswordHasher};
pub fn hash_string(password: &[u8; 13], salt: &[u8; 16]) -> Result<[u8; 32], Error> {
    let mut output_key = [0u8; 32];
    Argon2::default().hash_password_into(password, salt, &mut output_key).expect("Could not encrypt key");
    return Ok(output_key)
}