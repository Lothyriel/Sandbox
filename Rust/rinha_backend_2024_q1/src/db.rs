use async_sqlite::rusqlite::{Connection, Error, Transaction};

use crate::models::*;

pub fn seed_data(conn: &Connection, data: &[(u32, u32)]) -> Result<(), Error> {
    for (id, limit) in data {
        conn.execute(
            "INSERT OR IGNORE INTO clients (id, debit_limit, balance) VALUES (?1, ?2, ?3)",
            (id, limit, 0),
        )?;
    }

    Ok(())
}

pub fn grant_database_tables(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clients (
             id INTEGER PRIMARY KEY,
             debit_limit INTEGER NOT NULL,
             balance INTEGER NOT NULL
        );",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             client_id INTEGER NOT NULL,
             value INTEGER NOT NULL,
             type TEXT NOT NULL,
             description TEXT NOT NULL,
             date DATETIME NOT NULL
        );",
        (),
    )?;

    Ok(())
}

pub fn insert_transaction_data(
    tx: &Transaction,
    client_id: u32,
    request: TransactionRequest,
    new_balance: i64,
) -> Result<(), Error> {
    tx.execute(
        "INSERT INTO transactions (
            client_id,
            value,
            type,
            description,
            date
         ) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            client_id,
            request.value,
            request.transaction_type,
            request.description,
            chrono::Utc::now(),
        ),
    )?;

    tx.execute(
        "UPDATE clients SET balance = (?1) WHERE id = (?2);",
        (new_balance, client_id),
    )?;

    Ok(())
}

pub fn get_extract_data(
    conn: &Connection,
    client_id: u32,
    client: ClientData,
) -> Result<ExtractResponse, Error> {
    let mut query = conn.prepare(
        "SELECT value, type, description, date
         FROM transactions
         WHERE client_id = (?)
         ORDER BY date DESC
         LIMIT 10;",
    )?;

    let transactions: Result<_, _> = query
        .query_map([client_id], |row| {
            Ok(TransactionData {
                value: row.get(0)?,
                transaction_type: row.get(1)?,
                description: row.get(2)?,
                date: row.get(3)?,
            })
        })?
        .collect();

    Ok(ExtractResponse {
        balance: ExtractData {
            total: client.balance,
            date: chrono::Utc::now(),
            limit: client.limit,
        },
        transactions: transactions?,
    })
}
