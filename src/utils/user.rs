pub struct User {
    pub username: String,
    pub master_key: String,
    pub sym_key: String,
    pub private_key: String,
}

impl User {
    // Méthode pour créer un nouvel utilisateur
    pub fn new(username: &str, master_key: &str, sym_key: &str, private_key: &str) -> Self {
        User {
            username: String::from(username),
            master_key: String::from(master_key),
            sym_key: String::from(sym_key),
            private_key: String::from(private_key),
        }
    }
}