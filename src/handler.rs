use actix_web::{
    get, post,
    web::{Data, Json, Path},
};

use crate::{
    client_store::{Client, ClientBalance, ClientId, ClientStore, ClientStoreError},
    requests::{NewClientRequest, NewCreditTransactionRequest, NewDebitTransactionRequest},
};

// POST requests

/// Create a new client and return it's client id
#[post("/new_client")]
async fn new_client(
    data: Data<ClientStore>,
    request: Json<NewClientRequest>,
) -> Result<Json<ClientId>, ClientStoreError> {
    Ok(Json(data.new_client(&request.info).await?))
}

/// Credit an existing client's account and return the updated balance
#[post("/new_credit_transaction")]
async fn new_credit_transaction(
    data: Data<ClientStore>,
    request: Json<NewCreditTransactionRequest>,
) -> Result<Json<ClientBalance>, ClientStoreError> {
    Ok(Json(
        data.credit_client(request.client_id, request.credit_amount)
            .await?,
    ))
}

/// Debit an existing client's account and return the updated balance
#[post("/new_debit_transaction")]
async fn new_debit_transaction(
    data: Data<ClientStore>,
    request: Json<NewDebitTransactionRequest>,
) -> Result<Json<ClientBalance>, ClientStoreError> {
    Ok(Json(
        data.debit_client(request.client_id, request.debit_amount)
            .await?,
    ))
}

/// Write all current client's balances to a balance report file and clear all in-memory balances
#[post("/store_balances")]
async fn store_balances(data: Data<ClientStore>) -> Result<(), ClientStoreError> {
    data.store_balances().await
}

// GET requests

/// Return a client's information and current balance
#[get("/client_balance/{user_id}")]
async fn client_balance(
    data: Data<ClientStore>,
    client_id: Path<ClientId>,
) -> Result<Json<Client>, ClientStoreError> {
    Ok(Json(data.get_client(*client_id).await?))
}
