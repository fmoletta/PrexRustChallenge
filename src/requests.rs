//! Struct definitions for Json-Typed request bodies
use crate::client_store::{ClientBalance, ClientId, ClientInfo};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewClientRequest {
    #[serde(flatten)]
    pub info: ClientInfo,
}

#[derive(Deserialize)]
pub struct NewCreditTransactionRequest {
    pub client_id: ClientId,
    pub credit_amount: ClientBalance,
}

#[derive(Deserialize)]
pub struct NewDebitTransactionRequest {
    pub client_id: ClientId,
    pub debit_amount: ClientBalance,
}
