Prex Rust Challenge: Payment Processor Service
====

This project consists of a simplistic payment processor service exposed as a REST API.

Usage:
In order to launch the service you can run:
```=bash
  cargo run --release
```
Which will launch the service at address http://127.0.0.1:8080 (This can be changed by modifying the constant SERVICE_ADDRESS if needed)
Once launched, the service will respond to the folliwing endpoints:

POST endpoints:

* new_client:

  This endpoint will create a new client with default balance. There musn't be a client already present with the same document number. The service will respond with the client's id.
  This endpoint must receive the following json payload:
  ```=json
  {
    "client_name": <String>,
    "birth_date": <NaiveDate>,
    "document_number": <String>,
    "country": <String>,
  }
  ```

* new_credit_transaction:

  This endpoint will increase the client's balance by the given credit amount. The credit amount must not be negative and the client must already exist. The service will respond with the updated balance.
  This endpoint must receive the following json payload:
  ```=json
  {
    "client_id": <Int>,
    "credit_amount": <Decimal>,
  }
  ```

* new_debit_transaction:

  This endpoint will decrease the client's balance by the given debit amount. The debit amount must not be negative and the client must already exist. The service will respond with the updated balance.
  This endpoint must receive the following json payload:
  ```=json
  {
    "client_id": <Int>,
    "debit_amount": <Decimal>,
  }
  ```

* store_balances

  This endpoint will write all current client's balances to a balance report file. The balance report file will be created on the `balance_reports` directory, with the name YYMMDD_FILE_NO.DAT where FILE_NO is the counter for each file created since the service was launched. The file's contents will consist of each client's id and balance separated by a whitespace with a newline for each client. As a result of this operation all client's in memory balances will be set to zero. This endpoint does not require any additional payloads

GET endpoints:

* get_client_balance:

  This endpoint will return the given client's information and current balance as a json payload with the following format:
  ```=json
  {
    "client_name": <String>,
    "birth_date": <NaiveDate>,
    "document_number": <String>,
    "country": <String>,
    "balance": <Decimal>
  }
  ```
The client must already exist and its client id must be provided via url argument.

For usage examples please check out the [Postman Collection](PrexRustChallenge.postman_collection.json)

Service Limitations:

As this is a simplistic version of a payment processor, client data is kept in-memory, which limits the amount of clients the service can keep track of for a given instance, and provides no way of recovering previous client information in-between service restarts.
