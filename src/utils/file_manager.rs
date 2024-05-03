use std::{env, fs};
use crate::utils::utils;
use crate::utils::crypto_utils;
use crate::utils::user::User;
use crate::utils::folder::Folder;
use crate::server;
use crate::consts::HOME_DIR;
use crate::consts::SHARED_FOLDER;

pub fn get_root_folder(user: &User) -> Folder {
    let mut folders = Vec::new();
    folders.push(user.username.clone());
    folders.push(SHARED_FOLDER.to_string());

    let mut enc_folders = Vec::new();
    enc_folders.push(user.username.clone());
    enc_folders.push(SHARED_FOLDER.to_string());

    let folder = Folder::new(HOME_DIR, "", HOME_DIR, "",  &user.sym_key, &Vec::new(), &Vec::new(), &folders, &enc_folders, &Vec::new(), &Vec::new());
    folder
}

pub fn get_folder(folder_name: &str, folder: &Folder, user: &User) -> Folder {

    if folder_name == ".." {
        let enc_path = &folder.enc_path;

        let mut vec: Vec<&str> = enc_path.split("/").collect();

        vec.pop();

        let mut folder = get_root_folder(&user);

        if vec.len() == 0 {
            return folder;
        }

        vec.remove(0);

        if vec.len() == 0 {
            return folder;
        }

        if vec[0] != user.username  {

            let (shared_folders_name, shared_folders_path, _shared_folders_sym_key) = get_shared_folders_info(&user);
            // found folder.enc_path in shared_folders_path
            let mut count = 0;
            let mut index = 0;
            let mut nb_same_char = 0;
            for sf_path in &shared_folders_path {
                if enc_path.contains(sf_path) {
                    let mut e_p = enc_path.clone();
                    e_p = e_p.replace(sf_path, "");
                    let nb = e_p.len();
                    if nb > nb_same_char {
                        nb_same_char = nb;
                        index = count;
                    }
                    break;
                }
                count += 1;
            }

            let ep = enc_path.replace(&shared_folders_path[index], "");

            let mut vec = Vec::new();

            vec.push(shared_folders_name[index].clone());

            let mut tmp_vec: Vec<&str> = ep.split("/").collect();

            tmp_vec.remove(0);
            tmp_vec.remove(0);

            for v in tmp_vec {
                vec.push(v.to_string());
            }

            vec.pop();

            folder = get_folder(SHARED_FOLDER, &folder, &user);

            if vec.len() == 0 {
                return folder;
            }

            folder = get_folder(&vec.remove(0), &folder, &user);

            if vec.len() == 0 {
                return folder;
            }

            loop {
                let next_enc_folder = vec.remove(0);
                let next_folder = &folder.folders[folder.enc_folders.iter().position(|x| *x == next_enc_folder).unwrap()];
                folder = get_folder(next_folder, &folder, &user);
                if vec.len() == 0 {
                    return folder;
                }
            }

        } else {
            loop {
                let next_enc_folder = vec.remove(0);
                let next_folder = &folder.folders[folder.enc_folders.iter().position(|x| *x == next_enc_folder).unwrap()];
                folder = get_folder(next_folder, &folder, &user);
                if vec.len() == 0 {
                    return folder;
                }
            }
        }
    }

    let enc_name = &folder.enc_folders[folder.folders.iter().position(|x| *x == folder_name).unwrap()];

    let enc_path = format!("{}/{}", folder.enc_path, enc_name);

    let vec: Vec<&str> = enc_path.split("/").collect();

    if vec.len() == 1 {
        let folder = get_root_folder(&user);
        folder
    } else if vec.len() == 2 {
        if vec.last().unwrap() == &user.username {
            let name = &user.username;
            let enc_name = &user.username;
            let sym_key = user.sym_key.clone();
            let path = HOME_DIR.to_owned() + "/" + &user.username;
            let enc_path = "/".to_string() + &user.username;

            let(files_name, enc_files_name, folders_name, enc_folders_name, folder_sym_key) = get_files_and_folders(&enc_path, &sym_key);

            let folder = Folder::new(name, enc_name, &path, &enc_path, &sym_key, &files_name, &enc_files_name, &folders_name, &enc_folders_name, &folder_sym_key, &Vec::new());

            folder
        } else {

            let (shared_folders_name, shared_folders_path, shared_folders_sym_key) = get_shared_folders_info(&user);
            let path = format!("{}/{}", HOME_DIR, SHARED_FOLDER);
            let enc_path = "";
            let sym_key = &user.sym_key;
            let folder = Folder::new(folder_name, folder_name, &path, enc_path, sym_key, &Vec::new(), &Vec::new(), &shared_folders_name, &shared_folders_path, &shared_folders_sym_key, &Vec::new());
            folder
        }
    } else {
        let path = format!("{}/{}", folder.path, folder_name);
        let enc_path = format!("{}/{}", folder.enc_path, enc_name);
        let sym_key = folder.folder_sym_key[folder.folders.iter().position(|x| *x == folder_name).unwrap()].clone();
        let (files_name, enc_files_name, folders_name, enc_folders_name, folder_sym_key) = get_files_and_folders(&enc_path, &sym_key);
        let users_shared_with = get_shared_with(&enc_path, &sym_key);
        println!("users_shared_with: {:?}", users_shared_with);
        let folder = Folder::new(folder_name, enc_name, &path, &enc_path, &sym_key, &files_name, &enc_files_name, &folders_name, &enc_folders_name, &folder_sym_key, &users_shared_with);
        folder
    }
}

fn get_shared_folders_info(user: &User) -> (Vec<String>, Vec<String>, Vec<String>) {
    let shared_folders = get_shared_folders(&user);
    let mut shared_folders_name = Vec::new();
    let mut shared_folders_sym_key = Vec::new();
    let mut shared_folders_path = Vec::new();
    for shared_folder in shared_folders {
        shared_folders_path.push(shared_folder.0.clone()[1..shared_folder.0.len()].to_string());
        let folder_enc_name = shared_folder.0.split("/").last().unwrap();
        let folder_name = crypto_utils::decrypt_symmetric(folder_enc_name, &utils::get_nonce(&shared_folder.0), &shared_folder.1);
        shared_folders_sym_key.push(shared_folder.1.clone());
        shared_folders_name.push(folder_name);
    }
    (shared_folders_name, shared_folders_path, shared_folders_sym_key)
}

fn get_shared_folders(user: &User) -> Vec<(String, String)> {

    let mut shared_folders = Vec::new();
    let shared_folders_enc = server::get_shared_folders_for_user(&user.username);
    for folder in shared_folders_enc {
        let folder_path = crypto_utils::decrypt_rsa(&folder.0, &user.private_key);
        let folder_sym_key = crypto_utils::decrypt_rsa(&folder.1, &user.private_key);
        shared_folders.push((folder_path, folder_sym_key));
    }

    // check if a folder and on its subfolder is shared, if so, remove the subfolder
    let mut shared_folders_to_remove = Vec::new();

    for i in 0..shared_folders.len() {
        for j in 0..shared_folders.len() {
            if i == j {
                continue;
            }
            if shared_folders[i].0.contains(&shared_folders[j].0) {
                println!("{} contains {}", shared_folders[i].0, shared_folders[j].0);
                shared_folders_to_remove.push(i);
            }
        }
    }

    shared_folders_to_remove.sort();

    shared_folders_to_remove.dedup();

    shared_folders_to_remove.reverse();

    let shared_folders_enc = server::get_shared_folders_for_user(&user.username);

    for i in shared_folders_to_remove {
        server::remove_shared_folder(&user.username, &shared_folders_enc[i].0);
        shared_folders.remove(i);
    }

    shared_folders
}

fn get_files_and_folders(path: &str, sym_key: &str) -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>) {

    let (enc_files, enc_folders) = utils::list_files_and_directories(&path);

    let mut folders_name = Vec::new();
    let mut enc_folders_name = Vec::new();
    let mut folders_sym_key = Vec::new();

    for f in &enc_folders {
        let (enc_sym_key, sym_key_nonce) = utils::get_sym_key_and_nonce(f);
        let sym_key = crypto_utils::decrypt_symmetric(&enc_sym_key, &sym_key_nonce, &sym_key);
        folders_sym_key.push(sym_key.clone());
        let nonce = utils::get_nonce(f);
        enc_folders_name.push(f.split("/").last().unwrap().trim().to_string());
        let folder_name = crypto_utils::decrypt_symmetric(enc_folders_name.last().unwrap().trim(), &nonce, &sym_key);
        folders_name.push(folder_name);
    }

    let mut files_name = Vec::new();
    let mut enc_files_name = Vec::new();

    for f in &enc_files {
        let nonce = utils::get_nonce(f);
        enc_files_name.push(f.split("/").last().unwrap().trim().to_string());
        let file_name = crypto_utils::decrypt_symmetric(enc_files_name.last().unwrap().trim(), &nonce, &sym_key);
        files_name.push(file_name);
    }

    (files_name, enc_files_name, folders_name, enc_folders_name, folders_sym_key)
}

pub fn create_folder(folder_name: &str, folder: &Folder, user: &User) -> Folder {

    if folder.path == HOME_DIR ||
        folder.path == HOME_DIR.to_owned() + "/" + SHARED_FOLDER ||
        folder.folders.contains(&String::from(folder_name)) ||
        !utils::check_foldername(folder_name) {
        let folder = Folder::get_empty_folder();
        folder
    } else {

        let sym_key = crypto_utils::generate_sym_key();

        let (enc_folder_name, folder_nonce) = crypto_utils::encrypt_symmetric(&folder_name, &sym_key);

        let (enc_sym_key, sym_key_nonce) = crypto_utils::encrypt_symmetric(&sym_key, &folder.sym_key);

        let mut shared_with = Vec::new();

        println!("shared with: {:?}", folder.shared_with);

        for u in &folder.shared_with {
            if u == &user.username {
                continue;
            }
            let (enc_user, nonce) = crypto_utils::encrypt_symmetric(u, &sym_key);
            let enc_user = nonce + ":" + &enc_user;
            shared_with.push(enc_user);
        }

        let (enc_user, nonce) = crypto_utils::encrypt_symmetric(&user.username, &sym_key);
        let enc_user = nonce + ":" + &enc_user;
        if !shared_with.contains(&enc_user) {
            shared_with.push(enc_user);
        }

        server::create_folder(&enc_folder_name, &folder.enc_path, &folder_nonce, &enc_sym_key, &sym_key_nonce, &shared_with);

        let mut folders = Vec::clone(&folder.folders);

        folders.push(String::from(folder_name));

        let mut enc_folders = Vec::clone(&folder.enc_folders);

        enc_folders.push(enc_folder_name.clone());

        let mut folders_sym_key = Vec::clone(&folder.folder_sym_key);

        folders_sym_key.push(sym_key);

        let folder = Folder::new(&folder.name, &folder.enc_name, &folder.path, &folder.enc_path, &folder.sym_key, &folder.files, &folder.enc_files, &folders, &enc_folders, &folders_sym_key, &shared_with);

        folder
    }
}

fn add_shared_with_for_every_folder(enc_path: &str, sym_key: &str, username: &str) {

    let (enc_username, nonce) = crypto_utils::encrypt_symmetric(username, &sym_key);

    let enc_username = nonce + ":" + &enc_username;

    server::add_shared_with(enc_path, enc_username.as_str());

    let(_files_name, _enc_files_name, _folders_name, enc_folders_name, folder_sym_key) = get_files_and_folders(enc_path, sym_key);

    for folder in &enc_folders_name {
        let enc_path = format!("{}/{}", enc_path, folder);
        add_shared_with_for_every_folder(&enc_path, &folder_sym_key[enc_folders_name.iter().position(|x| &x == &folder).unwrap()], username);
    }
}

pub fn share_folder(folder: &Folder, user: &User, username: &str) -> Folder {

    if !folder.path.contains(&(HOME_DIR.to_owned() + "/" + &user.username + "/")) {
        return Folder::get_empty_folder();
    }

    let user_pk = server::get_user_pk(username);

    let enc_sym_key = crypto_utils::encrypt_rsa(&folder.sym_key, &user_pk);

    let enc_path = crypto_utils::encrypt_rsa(&folder.enc_path, &user_pk);

    add_shared_with_for_every_folder(&folder.enc_path, &folder.sym_key, username);

    if !server::add_shared_folder(username, &enc_sym_key, &enc_path) {
        return Folder::get_empty_folder();
    }

    let mut shared_with = Vec::clone(&folder.shared_with);

    shared_with.push(username.to_string());

    let folder = Folder::new(&folder.name, &folder.enc_name, &folder.path, &folder.enc_path, &folder.sym_key, &folder.files, &folder.enc_files, &folder.folders, &folder.enc_folders, &folder.folder_sym_key, &shared_with);

    folder
}

pub fn get_shared_with(path: &str, sym_key: &str) -> Vec<String> {

    let mut users = server::get_folder_shared_with(path);

    let mut users_name = Vec::new();

    for user in users {
        let nonce_and_user = user.split(":").collect::<Vec<&str>>();
        let nonce = nonce_and_user[0];
        let user = nonce_and_user[1];
        let user = crypto_utils::decrypt_symmetric(user, &nonce, &sym_key);
        users_name.push(user);
    }

    users_name

}

pub fn upload_file(path: &str, folder: &Folder) -> Folder {

    if !utils::check_file(path) {
        return Folder::get_empty_folder();
    }

    let file_name = path.split("/").last().unwrap();

    if folder.path == HOME_DIR ||
        folder.path == HOME_DIR.to_owned() + "/" + SHARED_FOLDER ||
        folder.files.contains(&file_name.to_string()) {
        return Folder::get_empty_folder();
    }

    let sym_key = &folder.sym_key;

    let (enc_name, nonce_name) = crypto_utils::encrypt_symmetric(file_name, &sym_key);

    let file_data = fs::read(path).unwrap();

    let (enc_data, nonce_data) = crypto_utils::encrypt_symmetric_bytes(&file_data, &sym_key);

    if !server::upload_file(&enc_name, &enc_data, &folder.enc_path, &nonce_name, &nonce_data) {
        return Folder::get_empty_folder();
    }

    let mut files = Vec::clone(&folder.files);
    files.push(file_name.to_string());

    let mut enc_files = Vec::clone(&folder.enc_files);

    enc_files.push(enc_name.clone());

    let folder = Folder::new(&folder.name, &folder.enc_name, &folder.path, &folder.enc_path, &folder.sym_key, &files, &enc_files, &folder.folders, &folder.enc_folders, &folder.folder_sym_key, &folder.shared_with);

    folder
}

pub fn download_file(filename: &str, folder: &Folder) -> bool {

    let enc_filename = folder.enc_files[folder.files.iter().position(|x| *x == filename).unwrap()].clone();

    let file_path = format!("{}/{}", folder.enc_path, enc_filename);

    if !server::check_file(&file_path) {
        return false;
    }

    let file_data = server::download_file(&file_path);

    let nonce = utils::get_json_val(&file_path, "nonce_file");

    let sym_key = &folder.sym_key;

    let file_data = crypto_utils::decrypt_symmetric_bytes(&file_data, &nonce, &sym_key);

    let download_folder = env::var("HOME").unwrap() + "/Downloads";

    //create download folder if not exists
    if !fs::metadata(&download_folder).is_ok() {
        fs::create_dir(&download_folder).unwrap();
    }

    let download_path = format!("{}/{}", download_folder, filename);

    fs::write(&download_path, file_data).unwrap();

    true
}