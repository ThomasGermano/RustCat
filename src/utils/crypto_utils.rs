use hex;
use hkdf::Hkdf;
use rsa::pkcs8::der::zeroize::Zeroizing;
use rsa::pkcs8::DecodePrivateKey;
use rsa::pkcs8::DecodePublicKey;
use rsa::pkcs8::EncodePrivateKey;
use rsa::pkcs8::EncodePublicKey;
use rsa::{RsaPrivateKey, RsaPublicKey, Oaep};
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake128,
};
use sha3::{Digest, Sha3_256};

use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

use chacha20poly1305::aead::rand_core::RngCore;
use chacha20poly1305::consts::{U12, U32};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use sha3::digest::generic_array::GenericArray;

pub fn shake_128(input: &str) -> String {
    let mut hasher = Shake128::default();
    hasher.update(input.as_ref());
    let mut output = [0u8; 16];
    hasher.finalize_xof().read(&mut output);
    hex::encode(output)
}

pub fn create_asym_key() -> (Zeroizing<String>, String) {
    let mut rng = rand::thread_rng();
    let bits = 2048;

    // Generate a key pair in PKCS#1v2 format.
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // Convert key pair to PEM
    let priv_key_pem = priv_key
        .to_pkcs8_pem(Default::default())
        .expect("failed to convert to pem");
    let pub_key_pem = pub_key
        .to_public_key_pem(Default::default())
        .expect("failed to convert to pem");

    (priv_key_pem, pub_key_pem)
}

pub fn encrypt_rsa(payload: &str, public_key: &str) -> String {
    let mut rng = rand::thread_rng();
    let pub_key = RsaPublicKey::from_public_key_pem(public_key).unwrap();
    let padding = Oaep::new::<Sha3_256>();
    let enc_data = pub_key
        .encrypt(&mut rng, padding, payload.as_bytes())
        .expect("failed to encrypt data");
    hex::encode(enc_data)
}

pub fn decrypt_rsa(payload: &str, private_key: &str) -> String {
    let payload = hex::decode(payload).unwrap();
    let priv_key = RsaPrivateKey::from_pkcs8_pem(private_key).unwrap();
    let padding = Oaep::new::<Sha3_256>();
    let dec_data = priv_key
        .decrypt(padding, payload.as_slice())
        .expect("failed to decrypt data");
    String::from_utf8(dec_data).unwrap()
}

pub fn hash_password(password: &str, salt: &str) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::encode_b64(salt.as_bytes()).unwrap();
    let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
    hash.to_string()
}

pub fn hkdf_db(payload: &str) -> String {
    static INFO: &str = "db";
    let salt = b"";
    let hkdf = Hkdf::<Sha3_256>::new(Some(salt), payload.as_bytes());
    let mut arr = [0u8; 32];
    hkdf.expand(&INFO.as_bytes(), &mut arr).unwrap();
    hex::encode(arr)
}

pub fn stretched_master_key(payload: &str) -> String {
    static INFO: &str = "master_key";
    let salt = b"";
    let hkdf = Hkdf::<Sha3_256>::new(Some(salt), payload.as_bytes());
    let mut arr = [0u8; 32];
    hkdf.expand(&INFO.as_bytes(), &mut arr).unwrap();
    hex::encode(arr)
}

pub fn generate_sym_key() -> String {
    let mut sym_key = [0u8; 32];
    OsRng.fill_bytes(&mut sym_key);
    hex::encode(sym_key)
}

pub fn encrypt_symmetric(payload: &str, key: &str) -> (String, String) {
    let key = hex::decode(key).unwrap();
    let key: GenericArray<u8, U32> = *GenericArray::from_slice(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key.into());
    let ciphertext = cipher.encrypt(&nonce, payload.as_bytes()).unwrap();
    (hex::encode(ciphertext), hex::encode(nonce))
}

pub fn encrypt_symmetric_bytes(payload: &Vec<u8>, key: &str) -> (Vec<u8>, String) {
    let key = hex::decode(key).unwrap();
    let key: GenericArray<u8, U32> = *GenericArray::from_slice(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key.into());
    let ciphertext = cipher.encrypt(&nonce, payload.as_ref()).unwrap();
    (ciphertext, hex::encode(nonce))
}

// key as array of u8

pub fn decrypt_symmetric(payload: &str, nonce: &str, key: &str) -> String {
    let payload = hex::decode(payload).unwrap();
    let key = hex::decode(key).unwrap();
    let key: GenericArray<u8, U32> = *GenericArray::from_slice(&key);
    let nonce = hex::decode(nonce).unwrap();
    let nonce: GenericArray<u8, U12> = *GenericArray::from_slice(&nonce);
    let cipher = ChaCha20Poly1305::new(&key.into());
    let plaintext = cipher.decrypt(&nonce, payload.as_ref()).unwrap();
    String::from_utf8(plaintext).unwrap()
}

pub fn decrypt_symmetric_bytes(payload: &Vec<u8>, nonce: &str, key: &str) -> Vec<u8> {
    let key = hex::decode(key).unwrap();
    let key: GenericArray<u8, U32> = *GenericArray::from_slice(&key);
    let nonce = hex::decode(nonce).unwrap();
    let nonce: GenericArray<u8, U12> = *GenericArray::from_slice(&nonce);
    let cipher = ChaCha20Poly1305::new(&key.into());
    let plaintext = cipher.decrypt(&nonce, payload.as_ref()).unwrap();
    plaintext
}
