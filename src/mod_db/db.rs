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
    pub async fn get_connection(&mut self) {

        /*
        let mut config = Config::new();
        config.host(&self.server_name);
        config.database(&self.db_name);
        config.port(self.db_port);
        config.trust_cert();

        let res_token = get_azure_access_token_from(self.access_token.clone(),
                                                    Some(AZURE_SPN_DB_URL.to_string())).await;
        let res = match res_token {
            Ok(access_token) => {
                config.authentication(AuthMethod::AADToken(access_token.token.secret().to_string()));
                self.access_token = Some(access_token);
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        };
        match res {
            Ok(_) => {
                let tcp = TcpStream::connect(config.get_addr()).await;
                match tcp {
                    Ok(tcp) => {
                        tcp.set_nodelay(true).unwrap();
                        let mut client = Client::connect(config, tcp.compat_write()).await;
                    }
                    Err(e) => {

                    }
                }
            }
            Err(_) => {}
        }

         */
    }
}
