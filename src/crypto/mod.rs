use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, OsRng},
    AeadCore, Aes256Gcm, KeyInit,
};
use ring::{
    pbkdf2,
    rand::{SecureRandom, SystemRandom},
};
use std::num::NonZeroU32;

pub fn get_random_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    let r = SystemRandom::new();
    r.fill(&mut salt).unwrap();
    salt
}

pub fn derive_encryption_key(master_password: &str, salt: &[u8]) -> [u8; 32] {
    let mut enc_key: [u8; 32] = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(100_000).unwrap(),
        salt,
        master_password.as_bytes(),
        &mut enc_key,
    );
    enc_key
}

pub fn get_cipher(master_password: &str, salt: &[u8]) -> Aes256Gcm {
    let enc_key = derive_encryption_key(master_password, salt);
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&enc_key));
    cipher
}

pub fn encrypt_contents(contents: &str, master_password: &str, salt: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let cipher = get_cipher(master_password, salt);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let encrypted_text = cipher.encrypt(&nonce, contents.as_ref());
    (encrypted_text.unwrap(), nonce.to_vec())
}
