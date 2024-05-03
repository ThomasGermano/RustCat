use crate::utils::crypto_utils;
use crate::utils::user::User;
use crate::server;
pub fn login_user(username: &str, password: &str) -> User {

    let hash_password = generate_master_key(&username, &password);

    let h_pwd_db = crypto_utils::hkdf_db(&hash_password);

    let user = server::login_user(&username, &h_pwd_db);

    if user.is_null() {
        return User::new("john_doe", "", "", "");
    }

    let stretched_master_key = crypto_utils::stretched_master_key(&hash_password);

    let nouce_sym_key = user["protected_sym_key"].as_str().unwrap().split(":").collect::<Vec<&str>>()[0];

    let enc_sym_key = user["protected_sym_key"].as_str().unwrap().split(":").collect::<Vec<&str>>()[1];

    let sym_key = crypto_utils::decrypt_symmetric(&enc_sym_key, &nouce_sym_key, &stretched_master_key);

    let nouce_priv_key = user["protected_priv_key"].as_str().unwrap().split(":").collect::<Vec<&str>>()[0];

    let enc_priv_key = user["protected_priv_key"].as_str().unwrap().split(":").collect::<Vec<&str>>()[1];

    let priv_key = crypto_utils::decrypt_symmetric(&enc_priv_key, &nouce_priv_key, &sym_key);

    User::new(&username, &stretched_master_key, &sym_key, &priv_key)
}

pub fn register_user(username: &str, password: &str) -> User {

    let (priv_key, pub_key) = crypto_utils::create_asym_key();

    let hash_password = generate_master_key(&username, &password);

    let h_pwd_db = crypto_utils::hkdf_db(&hash_password);

    let stretched_master_key = crypto_utils::stretched_master_key(&hash_password);

    // generate 32 bytes key for sym encryption
    let sym_key = crypto_utils::generate_sym_key();

    let (enc_sym_key, nonce) = crypto_utils::encrypt_symmetric(&sym_key, &stretched_master_key);

    let enc_sym_key = nonce + ":" + &enc_sym_key;

    let (enc_priv_key, nonce_priv_key) = crypto_utils::encrypt_symmetric(&priv_key, &sym_key);

    let enc_priv_key = nonce_priv_key + ":" + &enc_priv_key;

    if !server::create_user(&username, &h_pwd_db, &enc_sym_key, &enc_priv_key, &pub_key) {
        return User::new("john_doe", "", "", "");
    }

    return User::new(&username, &stretched_master_key, &sym_key, &priv_key);
}

pub fn change_password(new_password: &str, user: &User) -> User {

        let hash_password = generate_master_key(&user.username, &new_password);

        let h_pwd_db = crypto_utils::hkdf_db(&hash_password);

        let stretched_master_key = crypto_utils::stretched_master_key(&hash_password);

        // generate 32 bytes key for sym encryption
        let sym_key = user.sym_key.clone();

        let (enc_sym_key, nonce) = crypto_utils::encrypt_symmetric(&sym_key, &stretched_master_key);

        let enc_sym_key = nonce + ":" + &enc_sym_key;

        if !server::change_password(&user.username, &h_pwd_db, &enc_sym_key) {
            return User::new("john_doe", "", "", "");
        }

        return User::new(&user.username, &stretched_master_key, &sym_key, &user.private_key);

}

fn generate_master_key(username: &str, password: &str) -> String {
    let hash_username = crypto_utils::shake_128(&username);
    let hash_password = crypto_utils::hash_password(&password, &hash_username);
    hash_password
}