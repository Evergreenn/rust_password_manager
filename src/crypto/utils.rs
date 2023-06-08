use std::fs;

use anyhow::anyhow;
// use chacha20poly1305::XChaCha20Poly1305;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

pub fn encrypt_small_file(
    filepath: &str,
    dist: &str,
    key: &[u8; 32],
    // nonce: &[u8; 24],
) -> Result<(), anyhow::Error> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let file_data = fs::read(filepath)?;

    let nonce = Nonce::from_slice("0123456789ab".as_bytes());

    let encrypted_file = cipher
        .encrypt(&nonce, file_data.as_ref())
        .map_err(|err| anyhow!("Encrypting small file: {err}"))?;

    fs::write(&dist, encrypted_file)?;

    Ok(())
}

pub fn decrypt_small_file(
    encrypted_file_path: &str,
    dist: &str,
    key: &[u8; 32],
) -> Result<(), anyhow::Error> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let file_data = fs::read(encrypted_file_path)?;

    let nonce = Nonce::from_slice("0123456789ab".as_bytes());

    let decrypted_file = cipher
        .decrypt(&nonce, file_data.as_ref())
        .map_err(|err| anyhow!("Decrypting small file: {}", err))?;

    fs::write(&dist, decrypted_file)?;

    Ok(())
}

//Write doctest
//
/// Generate a key from a password
/// ```
/// use crate::crypto::utils::gen_key_from_password;
///
/// let password = "password".to_string();
/// let key = gen_key_from_password(password);
/// assert_eq!(key.len(), 32);
/// ```
///
/// # Arguments
/// * `password` - A password
/// # Returns
/// * A key
///
pub fn gen_key_from_password(password: String) -> [u8; 32] {
    //TODO: put all this in a config file
    let salt = b"salt";
    let n = 4096;

    let mut key = [0u8; 32];

    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, n, &mut key);

    key
}
