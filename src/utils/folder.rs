#[derive(Debug, Clone)]
pub struct Folder {
    pub name: String,
    pub enc_name: String,
    pub path: String,
    pub enc_path: String,
    pub sym_key: String,
    pub files: Vec<String>,
    pub enc_files: Vec<String>,
    pub folders: Vec<String>,
    pub enc_folders: Vec<String>,
    pub folder_sym_key: Vec<String>,
    pub shared_with: Vec<String>,
}

impl Folder {
    pub fn new(name: &str, enc_name: &str, path: &str, enc_path: &str, sym_key: &str, files: &Vec<String>, enc_files: &Vec<String>, folders: &Vec<String>, enc_folders: &Vec<String>, folder_sym_key: &Vec<String>, shared_with: &Vec<String>) -> Self {

        Folder {
            name: String::from(name),
            enc_name: String::from(enc_name),
            path: String::from(path),
            enc_path: String::from(enc_path),
            sym_key: String::from(sym_key),
            files: Vec::clone(files),
            enc_files: Vec::clone(enc_files),
            folders: Vec::clone(folders),
            enc_folders: Vec::clone(enc_folders),
            folder_sym_key: Vec::clone(folder_sym_key),
            shared_with: Vec::clone(shared_with),
        }
    }

    pub fn clone(&self) -> Self {
        Folder {
            name: String::from(&self.name),
            enc_name: String::from(&self.enc_name),
            path: String::from(&self.path),
            enc_path: String::from(&self.enc_path),
            sym_key: String::from(&self.sym_key),
            files: Vec::clone(&self.files),
            enc_files: Vec::clone(&self.enc_files),
            folders: Vec::clone(&self.folders),
            enc_folders: Vec::clone(&self.enc_folders),
            folder_sym_key: Vec::clone(&self.folder_sym_key),
            shared_with: Vec::clone(&self.shared_with),
        }
    }

    pub fn get_empty_folder() -> Self {
        Folder {
            name: String::from(""),
            enc_name: String::from(""),
            path: String::from(""),
            enc_path: String::from(""),
            sym_key: String::from(""),
            files: Vec::new(),
            enc_files: Vec::new(),
            folders: Vec::new(),
            enc_folders: Vec::new(),
            folder_sym_key: Vec::new(),
            shared_with: Vec::new(),
        }
    }
}