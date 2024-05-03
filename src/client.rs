use std::io;
use std::io::Read;
use crate::utils::file_manager;
use crate::utils::user_manager;
use crate::utils::folder::Folder;
use crate::utils::user::User;
use crate::server;
use cliclack::{intro, input, select, outro, password, spinner, clear_screen, log, note};


static HOME_DIR: &str = "~";

pub fn welcome() {

    clear_screen().expect("");

    intro("Welcome to RustCat!").expect("");

    let choice = select("Login or register ?")
        .item("l", "Login", "")
        .item("r", "Register", "")
        .item("exit", "Exit", "")
        .interact().expect("");

    let mut user = User::new("", "", "", "");

    if choice == "l" {
        user = login();
    } else if choice == "r" {
        user = register();
    } else {
        exit();
    }

    let folder = file_manager::get_root_folder(&user);

    commands(user, folder);
}

// gérer les inputs utilisateurs pour les commandes
fn commands(user: User, folder: Folder) {

    clear_screen().expect("");

    intro(format!("Welcome to RustCat, {}!", user.username)).expect("");

    note("Current directory:", &folder.path).expect("");

    let command = select("What do you want to do?")
        .item("cd", "Change directory", "")
        .item("ls", "List directories and files", "")
        .item("uf", "Upload file", "")
        .item("df", "Download file", "")
        .item("rm", "Remove file", "")
        .item("cf", "Create folder", "")
        .item("sf", "Share folder", "")
        .item("rf", "Revoke access to folder", "")
        .item("cp", "Change password", "")
        .item("logout", "Logout", "")
        .item("exit", "Exit", "")
        .interact().expect("");

    if command == "cd" {
        cd(user, folder);
    } else if command == "ls" {
        ls(user, folder);
    } else if command == "uf" {
        uf(user, folder);
    } else if command == "df" {
        df(user, folder);
    } else if command == "rm" {
        rm(user, folder);
    } else if command == "cf" {
        cf(user, folder);
    } else if command == "sf" {
        sf(user, folder);
    } else if command == "rf" {
        rf(user, folder);
    } else if command == "cp" {
        cp(user, folder);
    } else if command == "logout" {
        logout();
    } else if command == "exit" {
        exit();
    } else {
        println!("Invalid input!");
        commands(user, folder);
    }
}

fn exit() {
    outro("Exit").expect("");
    std::process::exit(0);
}

fn logout() {
    welcome();
}

fn cp(user: User, folder: Folder) {

    clear_screen().expect("");

    intro("Change password").expect("");

    let mut pwd = password("Enter your old password: ").interact().expect("");

    loop {
        if user_manager::login_user(&user.username, &pwd).username != "john_doe" {
            break;
        }

        pwd = password("Wrong password! Enter your old password: ").interact().expect("");
    }

    let pwd = password("Enter your new password: ").interact().expect("");

    let mut spinner = spinner();

    spinner.start("Changing password...");

    let u = user_manager::change_password(&pwd, &user);

    if user.username == "john_doe" {
        spinner.stop("Can't change password!");
        pause();
        commands(user, folder);
    } else {
        spinner.stop("Password changed!");
        pause();
        commands(u, folder);
    }
}

fn rf(user: User, folder: Folder) {


    commands(user, folder);
    todo!()
}

fn sf(user: User, folder: Folder) {

    clear_screen().expect("");

    intro("Share folder").expect("");

    let users = server::list_user();

    let mut items: Vec<(&str, &str, &str)> = vec![];

    items.push((".", ".",""));

    println!("{:?}", &folder.shared_with);

    for u in &users {
        if u == &user.username || folder.shared_with.contains(&u.to_string()) {
            continue;
        }
        items.push((u, u, ""));
    }

    let items = items.as_slice();

    let choice = select("With who do you want to share this folder?").initial_value("").items(items).interact().expect("");

    if choice == "." {
        commands(user, folder);
        return;
    }

    let mut spinner = spinner();

    spinner.start(format!("Sharing folder with {}", choice));

    let backup_folder = folder.clone();
    let folder = file_manager::share_folder(&folder, &user, choice);

    if folder.name == "" {
        spinner.stop("Can't share this folder!");
        pause();
        commands(user, backup_folder);
    } else {
        spinner.stop("Folder shared!");
        pause();
        commands(user, folder);
    }

    return;
}

fn cf(user: User, folder: Folder) {

    clear_screen().expect("");

    intro("Create folder").expect("");

    let folder_name: String = input("Folder name: ").interact().expect("");

    let mut spinner = spinner();

    spinner.start("Creating folder...");

    let f = file_manager::create_folder(&folder_name, &folder, &user);

    if f.name == "" {
        spinner.stop("Can't create this folder!");
        pause();
        commands(user, folder);
    } else {
        spinner.stop("Folder created!");
        commands(user, f);
    }
}

fn rm(user: User, folder: Folder) {

    commands(user, folder);
    todo!()
}

fn df(user: User, folder: Folder) {

    clear_screen().expect("");

    intro("Download file").expect("");

    let mut items: Vec<(&str, &str, &str)> = vec![];

    items.push((".", ".",""));

    for f in &folder.files {
        items.push((f, f, ""));
    }

    let items = items.as_slice();

    let choice = select("Which file do you want to download?").initial_value("").items(items).interact().expect("");

    if choice == "." {
        commands(user, folder);
        return;
    }

    let mut spinner = spinner();

    spinner.start("Downloading file...");

    if file_manager::download_file(&choice, &folder) {
        spinner.stop("File downloaded in ~/Downloads!");
    } else {
        spinner.stop("Can't download this file!");
    }

    pause();

    commands(user, folder);
}

fn uf(user: User, folder: Folder) {

    clear_screen().expect("");

    intro("Upload file").expect("");

    let path: String = input("File path: ").interact().expect("");

    let mut spinner = spinner();

    spinner.start("Uploading file...");

    let f = file_manager::upload_file(&path, &folder);

    if f.name != "" {
        spinner.stop("File uploaded!");
        pause();
        commands(user, f);
    } else {
        spinner.stop("Can't upload this file!");
        pause();
        commands(user, folder);
    }
}

fn ls(user: User, folder: Folder) {

    clear_screen().expect("");

    let mut tree = "".to_string();

    // print all folders and files in current folder
    for f in &folder.folders {
        if f != folder.folders.last().unwrap() || folder.files.len() != 0 {
            tree = tree + &*format!("├{}\n", f);
        } else {
            tree = tree + &*format!("└{}\n", f);
        }
    }

    for f in &folder.files {
        if f != folder.files.last().unwrap() {
            tree = tree + &*format!("├─{}\n", f);
        } else {
            tree = tree + &*format!("└─{}\n", f);
        }
    }
    
    note(format!("Directories and files in {}", folder.name), tree).expect("");

    pause();

    commands(user, folder);
}

fn cd(user: User, folder: Folder) {

    clear_screen().expect("");

    intro("Change directory").expect("");

    let mut items: Vec<(&str, _, _)> = vec![];

    items.push((".", ".",""));
    if folder.path != "~" {
        items.push(("..", "..", ""));
    }

    for f in &folder.folders {
        items.push((f, f, ""));
    }

    // convert vec items to array
    let items = items.as_slice();

    let choice = select("Where do you want to go?").initial_value("").items(items).interact().expect("");

    if choice == "." {
        commands(user, folder);
    }  else {
        let folder = file_manager::get_folder(choice, &folder, &user);
        commands(user, folder);
    }
}

fn login() -> User {

    clear_screen().expect("");

    intro("Login").expect("");

    let (username, password) = prompt_for_username_and_password();

    let mut spinner = spinner();

    spinner.start("Logging in...");

    let user = user_manager::login_user(&username, &password);

    if user.username == "john_doe" {
        spinner.stop("Wrong username or password!");
        pause();
        welcome();
    }

    spinner.stop("Logged in!");

    user
}

fn register() -> User {
    
    clear_screen().expect("");

    intro("Register").expect("");

    let (username, password) = prompt_for_username_and_password();

    let mut spinner = spinner();

    spinner.start("Registering user...");

    let user = user_manager::register_user(&username, &password);

    if user.username == "john_doe" {
        spinner.stop("username already in use!");
        pause();
        welcome();
    }

    spinner.stop("User registered!");

    user
}

fn prompt_for_username_and_password() -> (String, String) {
    let username: String = input("Username: ").interact().expect("");
    let password: String = password("Password: ").interact().expect("");
    (username, password)
}

fn pause() {
    log::info("Press enter to continue...").expect("");
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}