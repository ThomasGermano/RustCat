use std::fs;
use std::io::{Read, Write};
use serde_json::{json, Value};
use serde_json::Value::Object;
use crate::consts::{ROOT_DIR, SHARED_FOLDER};
use crate::utils::utils;
pub fn create_user(username: &str, hash_password:
                    &str, enc_sym_key: &str,
                   enc_priv_key: &str, pub_key: &str) -> bool {

    if !utils::check_username(username) {
        return false;
    }

    if list_user().contains(&username.to_string()) {
        return false;
    }

    // create json object for user

    let path = format!("{}{}", ROOT_DIR, username);

    fs::create_dir(&path).unwrap();

    let mut user = serde_json::Map::new();

    user.insert("master_pwd_hash".to_string(), Value::String(hash_password.to_string()));

    user.insert("protected_sym_key".to_string(), Value::String(enc_sym_key.to_string()));

    user.insert("protected_priv_key".to_string(), Value::String(enc_priv_key.to_string()));

    user.insert("public_key".to_string(), Value::String(pub_key.to_string()));

    let user = Object(user);

    // write json object to file
    let path = format!("{}.{}.json", ROOT_DIR, username);

    fs::write(&path, user.to_string()).unwrap();

    true
}

pub fn login_user(username: &str, hash_password: &str) -> Value {

    let mut username = username;

    if !utils::check_username(username) {
        println!("username not valid for check");
        username = "john_doe";
    }

    if !list_user().contains(&username.to_string()) {
        username = "john_doe";
    }

    println!("{}", username);

    let path = format!("{}.{}.json", ROOT_DIR, username);

    println!("path: {}", path);

    let user = fs::read_to_string(&path).unwrap();

    let user: Value = serde_json::from_str(&user).unwrap();

    let master_pwd_hash = user["master_pwd_hash"].as_str().unwrap();

    if master_pwd_hash == hash_password {
        user
    } else {
        Value::Null
    }
}

pub fn change_password(username: &str, hash_password: &str, enc_sym_key: &str) -> bool {

    let path = format!("{}.{}.json", ROOT_DIR, username);

    let user = fs::read_to_string(&path).unwrap();

    let mut user: Value = serde_json::from_str(&user).unwrap();

    user["master_pwd_hash"] = Value::String(hash_password.to_string());

    user["protected_sym_key"] = Value::String(enc_sym_key.to_string());

    fs::write(&path, user.to_string()).unwrap();

    true
}

pub fn create_folder(folder_name: &str, path: &str, folder_nonce: &str, enc_sym_key: &str, sym_key_nonce: &str, shared_with: &Vec<String>) -> bool {

        let path = format!("{}{}", ROOT_DIR, path);

        let folder_path = format!("{}/{}", path, folder_name);

        fs::create_dir(&folder_path).unwrap();

        let mut folder = serde_json::Map::new();

        folder.insert("nonce".to_string(), Value::String(folder_nonce.to_string()));

        folder.insert("enc_sym_key".to_string(), Value::String(enc_sym_key.to_string()));

        folder.insert("sym_key_nonce".to_string(), Value::String(sym_key_nonce.to_string()));

        let sh_with: Vec<Value> = shared_with.iter().map(|x| json!(x)).collect();

        folder.insert("shared_with".to_string(), Value::Array(sh_with));

        let folder = Object(folder);

        let path = format!("{}/.{}.json", path, folder_name);

        fs::write(&path, folder.to_string()).unwrap();

        true
}

pub fn list_user() -> Vec<String> {
    let mut users = Vec::new();
    let entries = fs::read_dir(ROOT_DIR).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let meta = entry.metadata().unwrap();
        if meta.is_dir() {
            let path = entry.path();
            let path = path.to_str().unwrap();
            let path = path.replace(ROOT_DIR, "");
            if path == "john_doe" {
                continue;
            }
            users.push(path.to_string());
        }
    }

    users
}

pub fn get_user_pk(username: &str) -> String {

    let users = list_user();

    if !users.contains(&username.to_string()) {
        return "".to_string()
    }

    let path = format!("{}.{}.json", ROOT_DIR, username);

    let user = fs::read_to_string(&path).unwrap();

    let user: Value = serde_json::from_str(&user).unwrap();

    let public_key = user["public_key"].as_str().unwrap();

    public_key.to_string()
}

pub fn get_folder_shared_with(path: &str) -> Vec<String> {
    let p = &path[1..path.len()];
    let mut v = p.split("/").collect::<Vec<&str>>();
    let folder_name = v.pop().unwrap();
    let pa = v.join("/");
    let p = format!("{}{}/.{}.json", ROOT_DIR, pa, folder_name);
    let folder = fs::read_to_string(&p).unwrap();
    let mut folder: Value = serde_json::from_str(&folder).unwrap();
    let shared_with = folder["shared_with"].as_array().unwrap();
    let mut users = Vec::new();
    for user in shared_with {
        println!("{}", user.as_str().unwrap());
        users.push(user.as_str().unwrap().to_string());
    }
    users
}

pub fn add_shared_with(path: &str, username: &str) -> bool {

    let p = &path[1..path.len()];
    let mut v = p.split("/").collect::<Vec<&str>>();
    let folder_name = v.pop().unwrap();
    let pa = v.join("/");
    let p = format!("{}{}/.{}.json", ROOT_DIR, pa, folder_name);
    println!("{}", p);
    let folder = fs::read_to_string(&p).unwrap();
    let mut folder: Value = serde_json::from_str(&folder).unwrap();
    let shared_with = folder["shared_with"].as_array_mut().unwrap();
    if !shared_with.contains(&json!(username)) {
        shared_with.push(json!(username));
    }
    fs::write(&p, folder.to_string()).unwrap();
    true
}

pub fn add_shared_folder(username: &str, enc_sym_key: &str, path: &str) -> bool {

    let users = list_user();

    if !users.contains(&username.to_string()) {
        return false;
    }

    let p = format!("{}.{}.json", ROOT_DIR, username);

    let user = fs::read_to_string(&p).unwrap();

    let user: Value = serde_json::from_str(&user).unwrap();

    let mut user: serde_json::Map<String, Value> = user.as_object().unwrap().clone();

    let shared_folder = json!({"enc_path": path, "enc_sym_key": enc_sym_key});

    if !user.contains_key(SHARED_FOLDER) {
        user.insert(SHARED_FOLDER.to_string(), json!([]));
    }

    let mut shared_folders = user[SHARED_FOLDER].as_array_mut().unwrap();

    shared_folders.push(shared_folder);

    let user = Object(user);

    fs::write(&p, user.to_string()).unwrap();

    true
}

pub fn remove_shared_folder(username: &str, path: &str) -> bool {
    let users = list_user();

    if !users.contains(&username.to_string()) {
        return false;
    }

    let p = format!("{}.{}.json", ROOT_DIR, username);

    let user = fs::read_to_string(&p).unwrap();

    let user: Value = serde_json::from_str(&user).unwrap();

    let mut user: serde_json::Map<String, Value> = user.as_object().unwrap().clone();

    if !user.contains_key(SHARED_FOLDER) {
        user.insert(SHARED_FOLDER.to_string(), json!([]));
    }

    let mut shared_folders = user[SHARED_FOLDER].as_array_mut().unwrap();

    let mut index = 0;

    for folder in shared_folders {
        let enc_path = folder["enc_path"].as_str().unwrap();
        if enc_path == path {
            break;
        }
        index += 1;
    }

    let mut shared_folders = user[SHARED_FOLDER].as_array_mut().unwrap();

    shared_folders.remove(index);

    let user = Object(user);

    fs::write(&p, user.to_string()).unwrap();

    true
}

pub fn get_shared_folders_for_user(username: &str) -> Vec<(String, String)> {

    let users = list_user();

    if !users.contains(&username.to_string()) {
        return Vec::new();
    }

    let p = format!("{}.{}.json", ROOT_DIR, username);

    let user = fs::read_to_string(&p).unwrap();

    let user: Value = serde_json::from_str(&user).unwrap();

    let mut user: serde_json::Map<String, Value> = user.as_object().unwrap().clone();

    if !user.contains_key(SHARED_FOLDER) {
        user.insert(SHARED_FOLDER.to_string(), json!([]));
    }

    let shared_folders = user[SHARED_FOLDER].as_array().unwrap();

    let mut folders = Vec::new();

    for folder in shared_folders {
        let enc_path = folder["enc_path"].as_str().unwrap();
        let enc_sym_key = folder["enc_sym_key"].as_str().unwrap();
        folders.push((enc_path.to_string(), enc_sym_key.to_string()));
    }

    folders
}

pub fn upload_file(filename: &str, file: &Vec<u8>, path: &str, nonce_name: &str, nonce_file: &str) -> bool {

    let p = format!("{}{}", ROOT_DIR, path);

    let p = format!("{}/{}", p, filename);

    let mut f = fs::File::create(&p).unwrap();

    f.write_all(file).unwrap();

    let mut file = serde_json::Map::new();

    file.insert("nonce".to_string(), Value::String(nonce_name.to_string()));

    file.insert("nonce_file".to_string(), Value::String(nonce_file.to_string()));

    let file = Object(file);

    let path = format!("{}{}", ROOT_DIR, path);

    let path = format!("{}/.{}.json", path, filename);

    fs::write(&path, file.to_string()).unwrap();

    true
}

pub fn check_file(file_path: &str) -> bool {
    let fp = &file_path[1..file_path.len()];

    let fp = format!("{}{}", ROOT_DIR, fp);

    return utils::check_file(&fp);
}

pub fn download_file(file_path: &str) -> Vec<u8> {

    let fp = &file_path[1..file_path.len()];

    let fp = format!("{}{}", ROOT_DIR, fp);

    let mut f = fs::File::open(fp).unwrap();

    let mut file = Vec::new();

    f.read_to_end(&mut file).unwrap();

    file
}