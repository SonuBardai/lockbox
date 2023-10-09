use crate::cli::commands::copy_to_clipboard;
use crate::cli::io::{print, read_terminal_input, MessageType};
use crate::store::PasswordStore;
use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, OsRng},
    AeadCore, Aes256Gcm, KeyInit,
};
use base32::Alphabet;
use qr_terminal::TermQrCode;
use ring::{
    pbkdf2,
    rand::{SecureRandom, SystemRandom},
};
use sha2::{Digest, Sha256};
use std::io::{BufRead, Write};
use std::num::NonZeroU32;
use totp_rs::{Algorithm, Secret, TOTP};

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

pub fn verify_totp<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    password_store: &PasswordStore,
) -> bool {
    let mut hasher = Sha256::new();
    let bs = password_store.get_mp().as_bytes();
    hasher.update(bs);
    let hash = hasher.finalize();
    let token = read_terminal_input(reader, writer, Some("Enter 2FA/totp code"));
    let totp = TOTP::new(
        Algorithm::SHA256,
        6,
        1,
        30,
        Secret::Raw(hash.to_vec()).to_bytes().unwrap(),
    )
    .unwrap();
    totp.check_current(&token).unwrap()
}

pub fn totp_init(master_password: &String) {
    let mut hasher = Sha256::new();
    let bs = master_password.as_bytes();
    hasher.update(bs);
    let hash = hasher.finalize();
    let b32 = base32::encode(Alphabet::RFC4648 { padding: false }, &hash);
    let totp_link = format!("otpauth://totp/Lockbox:lockbox?secret={}&issuer=Lockbox&digits=6&period=30&skew=1&algorithm=SHA256", b32);
    let mut output = std::io::stdout().lock();
    print(
        &mut output,
        &format!("Your totp url is: {}", totp_link),
        Some(MessageType::Info),
    );
    TermQrCode::from_bytes(totp_link.as_bytes()).print();
    println!();
    if copy_to_clipboard(totp_link).is_ok() {
        print(
            &mut output,
            "TOTP link copied to clipboard!",
            Some(MessageType::Info),
        );
    };

    /*match copy_to_clipboard(totp_link) {
        Ok(_) => {
            print(
                &mut output,
                "TOTP link copied to clipboard!",
                Some(MessageType::Info),
            );
        }
        Err(_) => {}
    };*/
}
