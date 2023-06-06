use std::fs;

use anyhow::anyhow;
// use chacha20poly1305::XChaCha20Poly1305;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};

pub fn encrypt_small_file(
    filepath: &str,
    dist: &str,
    key: &[u8; 32],
    // nonce: &[u8; 24],
) -> Result<(), anyhow::Error> {
    let cipher = ChaCha20Poly1305::new(key.into());

    let file_data = fs::read(filepath)?;

    let nonce = ChaCha20Poly1305::generate_nonce(OsRng);

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

    let nonce = ChaCha20Poly1305::generate_nonce(OsRng);

    let decrypted_file = cipher
        .decrypt(&nonce, file_data.as_ref())
        .map_err(|err| anyhow!("Decrypting small file: {}", err))?;

    fs::write(&dist, decrypted_file)?;

    Ok(())
}
