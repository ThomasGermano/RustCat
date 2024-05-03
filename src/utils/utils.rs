use std::fs;
use std::fs::File;
use std::io::Read;
use regex::Regex;
use crate::consts::ROOT_DIR;

pub fn check_username(username: &str) -> bool {
    // check with regex if username is valid
    let re = Regex::new(r"^[a-zA-Z0-9_]{3,16}$").unwrap();
    re.is_match(username)
}

pub fn check_foldername(folder_name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9_]{1,16}$").unwrap();
    re.is_match(folder_name)
}

pub fn list_files_and_directories(path: &str) -> (Vec<String>, Vec<String>) {
    let path = format!("{}{}", ROOT_DIR, path);
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    let entries = fs::read_dir(path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let meta = entry.metadata().unwrap();
        if meta.is_dir() {
            let path = entry.path();
            let path = path.to_str().unwrap();
            let path = path.replace(ROOT_DIR, "");
            dirs.push(path.to_string());
        } else {
            let path = entry.path();
            let path = path.to_str().unwrap();
            let filename = path.split("/").last().unwrap();
            if filename.starts_with(".") {
                continue;
            }
            let path = path.replace(ROOT_DIR, "");
            files.push(path.to_string());
        }
    }

    (files, dirs)
}

pub fn get_nonce(path: &str) -> String {
    get_json_val(path, "nonce")
}

pub fn get_json_val(path: &str, json_val: &str) -> String {
    let folder_name = path.split("/").last().unwrap();
    let last_part = "/".to_owned() + folder_name;
    let path = path.replace(&last_part.to_string(), "");
    let path = format!("{}{}/.{}.json", ROOT_DIR, path, folder_name);
    let folder = std::fs::read_to_string(&path).unwrap();
    let folder: serde_json::Value = serde_json::from_str(&folder).unwrap();
    folder[json_val].as_str().unwrap().to_string()
}

pub fn get_sym_key_and_nonce(path: &str) -> (String, String) {
    (get_json_val(path, "enc_sym_key"), get_json_val(path, "sym_key_nonce"))
}

pub fn check_file (path: &str) -> bool {
    //check if file exists

    return fs::metadata(path).is_ok();
}

pub fn get_file(filepath: &str) -> Vec<u8> {
    let mut f = File::open(filepath).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    buffer
}
