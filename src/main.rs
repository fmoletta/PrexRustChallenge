pub mod client_store;
pub mod handler;
pub mod requests;

use std::{fs::create_dir, path::Path};

use actix_web::{App, HttpServer, web::Data};

use crate::{
    client_store::ClientStore,
    handler::{
        client_balance, new_client, new_credit_transaction, new_debit_transaction, store_balances,
    },
};

/// Directory where balance reports created by `store_balances` shall be stored
pub const BALANCE_REPORTS_DIRECTORY: &str = "balance_reports";
/// Address on which the client will be launched
const SERVICE_ADDRESS: (&str, u16) = ("127.0.0.1", 8080);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file_dir_path = Path::new(BALANCE_REPORTS_DIRECTORY);
    if !file_dir_path.exists() {
        create_dir(file_dir_path)?;
    }
    let client_store = Data::new(ClientStore::new());
    HttpServer::new(move || {
        App::new()
            .app_data(client_store.clone())
            .service(new_client)
            .service(new_credit_transaction)
            .service(new_debit_transaction)
            .service(store_balances)
            .service(client_balance)
    })
    .bind(SERVICE_ADDRESS)?
    .run()
    .await
}
