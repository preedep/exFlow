use azure_core::auth::TokenResponse;

#[derive(Debug, Clone)]
pub struct Db {
    server_name: String,
    db_name: String,
    db_port: u16,
    access_token: Option<TokenResponse>,
}

impl Db {
    pub fn new(server_name: String, db_name: String, db_port: u16) -> Self {
        Db {
            server_name,
            db_name,
            db_port,
            access_token: None,
        }
    }
    pub fn get_connection(&self) {
        match &self.access_token {
            None => {}
            Some(access_token) => {}
        }
    }
}
