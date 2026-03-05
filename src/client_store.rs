use actix_web::ResponseError;
use chrono::{Local, NaiveDate};
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
};
use tokio::sync::{Mutex, RwLock};

use crate::BALANCE_REPORTS_DIRECTORY;

const DEFAULT_BALANCE: ClientBalance = Decimal::ZERO;

pub type ClientId = usize;
pub type ClientBalance = Decimal;

#[derive(Deserialize, Serialize, Clone)]
pub struct ClientInfo {
    client_name: String,
    birth_date: NaiveDate,
    document_number: String,
    country: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Client {
    #[serde(flatten)]
    info: ClientInfo,
    balance: ClientBalance,
}

#[derive(Default)]
pub struct ClientStore {
    // Client info & balances sorted by client id
    store: RwLock<Vec<Client>>,
    // A set of client documents checked to prevent duplicated document numbers across clients
    client_documents: Mutex<HashSet<String>>,
    // The next file number to be used when outputting balance report files
    next_file_number: AtomicUsize,
}

impl ClientStore {
    /// Create a new empty ClientStore
    pub fn new() -> ClientStore {
        ClientStore::default()
    }

    /// Creates a new client with default balance and returns its id
    /// Returns an error if there is a client present with the same document number or if there is no more space for new clients
    pub async fn new_client(&self, info: &ClientInfo) -> Result<ClientId, ClientStoreError> {
        // Check if we already have a client with the same document number
        let new_document_number = self
            .client_documents
            .lock()
            .await
            .insert(info.document_number.clone());
        if !new_document_number {
            return Err(ClientStoreError::DocumentNumberAlreadyPresent);
        }
        let mut store = self.store.write().await;
        let new_id = store.len();
        if new_id == ClientId::MAX {
            // Catch max client error before exceeding client capacity
            return Err(ClientStoreError::MaxClientsReached);
        }
        store.push(Client {
            info: info.clone(),
            balance: DEFAULT_BALANCE,
        });
        Ok(new_id)
    }

    /// Returns the client's info and current balance
    /// Returns an error if the client doesn't exist
    pub async fn get_client(&self, client_id: ClientId) -> Result<Client, ClientStoreError> {
        let store = self.store.read().await;
        if client_id >= store.len() {
            return Err(ClientStoreError::NonexistantClient(client_id));
        }
        Ok(store[client_id].clone())
    }

    /// Subtracts a given amount from the current client's balance and returns the updated balance
    /// Returns an error if the debit_amount is invalid, the client doesn't exist or if the operation underflows
    pub async fn debit_client(
        &self,
        client_id: ClientId,
        debit_amount: ClientBalance,
    ) -> Result<ClientBalance, ClientStoreError> {
        if debit_amount.is_sign_negative() {
            return Err(ClientStoreError::NegativeCreditDebitAmount);
        }
        let mut store = self.store.write().await;
        if client_id >= store.len() {
            return Err(ClientStoreError::NonexistantClient(client_id));
        }
        let new_balance = store[client_id]
            .balance
            .checked_sub(debit_amount)
            .ok_or(ClientStoreError::ClientBalanceUnderflow)?;
        store[client_id].balance = new_balance;
        Ok(new_balance)
    }

    /// Adds a given amount to the current client's balance and returns the updated balance
    /// Returns an error if the credit_amount is invalid, the client doesn't exist or if the operation overflows
    pub async fn credit_client(
        &self,
        client_id: ClientId,
        credit_amount: ClientBalance,
    ) -> Result<ClientBalance, ClientStoreError> {
        if credit_amount.is_sign_negative() {
            return Err(ClientStoreError::NegativeCreditDebitAmount);
        }
        let mut store = self.store.write().await;
        if client_id >= store.len() {
            return Err(ClientStoreError::NonexistantClient(client_id));
        }
        let new_balance = store[client_id]
            .balance
            .checked_add(credit_amount)
            .ok_or(ClientStoreError::ClientBalanceOverflow)?;
        store[client_id].balance = new_balance;
        Ok(new_balance)
    }

    /// Writes all current client balances to a balance report file and clears in-memory balances
    pub async fn store_balances(&self) -> Result<(), ClientStoreError> {
        // Use a write-lock so we don't update any client's balances while performing this operation
        let mut store = self.store.write().await;
        // Write client balances to a file & set each one to default
        let file_number = self.next_file_number.fetch_add(1, Ordering::Relaxed);
        let date = Local::now().date_naive().format("%Y%m%d");
        let filename = format! {"{date}_{file_number}.DAT"};
        let file = File::create_new(Path::new(BALANCE_REPORTS_DIRECTORY).join(filename))
            .map_err(|_| ClientStoreError::FileWrite)?;
        let mut file_writer = BufWriter::new(file);
        for (client_id, client) in store.iter_mut().enumerate() {
            let balance = client.balance;
            writeln!(&mut file_writer, "{client_id} {balance}")
                .map_err(|_| ClientStoreError::FileWrite)?;
            client.balance = DEFAULT_BALANCE;
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ClientStoreError {
    #[error("A client already exists for the given document number")]
    DocumentNumberAlreadyPresent,
    #[error("Maximum amount of clients reached")]
    MaxClientsReached,
    #[error("Client {0} not found")]
    NonexistantClient(ClientId),
    #[error("Client balance overflow")]
    ClientBalanceUnderflow,
    #[error("Client balance underflow")]
    ClientBalanceOverflow,
    #[error("Amounts for credit and debit operations must not be negative")]
    NegativeCreditDebitAmount,
    #[error("Failed to write to file")]
    FileWrite,
}

impl ResponseError for ClientStoreError {}
