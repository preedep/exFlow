use azure_core::auth::TokenResponse;
use log::{debug, error};
use tiberius::{AuthMethod, Client, Config, Query};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use crate::mod_azure::azure::get_azure_access_token_from;
use crate::mod_azure::entities::AZURE_SPN_DB_URL;
use crate::mod_db::entities::TblExFlowRuntimeClients;

pub async fn upsert_ex_flow_runtime_client(access_token: &TokenResponse, 
                                           runtime_client: &TblExFlowRuntimeClients) {
    
}
pub async fn get_employees() {
    let res_token = get_azure_access_token_from(None, Some(AZURE_SPN_DB_URL.to_string())).await;
    match res_token {
        Ok(token) => {
            debug!("Access Token : {:#?}", token);

            let mut config = Config::new();
            config.host("nickdatabaseserver001.database.windows.net");
            config.database("nickdatabaseserver001");
            config.port(1433);
            config.authentication(AuthMethod::AADToken(token.token.secret().to_string()));
            config.trust_cert();

            let tcp = TcpStream::connect(config.get_addr()).await;
            match tcp {
                Ok(tcp) => {
                    tcp.set_nodelay(true).unwrap();
                    let mut client = Client::connect(config, tcp.compat_write()).await;
                    match client {
                        Ok(mut c) => {
                            debug!("DB Connect success");
                            let mut select = Query::new("select * from Employee");
                            let res = select.query(&mut c).await;
                            debug!("Query completed");
                            debug!("reading rows");
                            match res {
                                Ok(r) => {
                                    debug!("Printing employee");
                                    let list = r.into_results().await.unwrap();
                                    for element in list.iter() {
                                        for row in element.iter() {
                                            debug!("{:?}", row);
                                        }
                                    }
                                }
                                Err(e) => {
                                    debug!("Query failed {:#?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("DB Connect failed: {:#?}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Connect to Server failed {:#?}", e);
                }
            }
        }
        Err(e) => {
            error!("{:#?}", e);
        }
    }
}
