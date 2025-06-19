use chrono::{DateTime, Utc};

use async_sqlite::rusqlite::{
    self,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    ToSql,
};

pub struct ClientData {
    pub id: ClientId,
    pub limit: u64,
    pub balance: i64,
}

pub type ClientId = u32;

#[derive(serde::Serialize, Debug)]
pub struct ExtractResponse {
    #[serde(rename = "saldo")]
    pub balance: ExtractData,
    #[serde(rename = "ultimas_transacoes")]
    pub transactions: Vec<TransactionData>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct TransactionRequest {
    #[serde(alias = "valor")]
    pub value: u64,
    #[serde(alias = "tipo")]
    pub transaction_type: TransactionType,
    #[serde(alias = "descricao")]
    pub description: String,
}

#[derive(serde::Serialize)]
pub struct TransactionResponse {
    #[serde(rename = "limite")]
    pub limit: u64,
    #[serde(rename = "saldo")]
    pub balance: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy)]
pub enum TransactionType {
    #[serde(rename = "d")]
    Debit,
    #[serde(rename = "c")]
    Credit,
}

#[derive(serde::Serialize, Debug)]
pub struct ExtractData {
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(rename = "data_extrato")]
    pub date: DateTime<Utc>,
    #[serde(rename = "limite")]
    pub limit: u64,
}

#[derive(serde::Serialize, Debug)]
pub struct TransactionData {
    #[serde(rename = "valor")]
    pub value: i64,
    #[serde(rename = "tipo")]
    pub transaction_type: TransactionType,
    #[serde(rename = "descricao")]
    pub description: String,
    #[serde(rename = "realizada_em")]
    pub date: DateTime<Utc>,
}

impl FromSql for TransactionType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Real(_) | ValueRef::Integer(_) | ValueRef::Null => {
                Err(FromSqlError::InvalidType)
            }
            ValueRef::Blob(e) | ValueRef::Text(e) => {
                match e.first().ok_or_else(|| FromSqlError::InvalidType)? {
                    b'd' => Ok(TransactionType::Debit),
                    b'c' => Ok(TransactionType::Credit),
                    _ => Err(FromSqlError::OutOfRange(e[0] as i64)),
                }
            }
        }
    }
}

impl ToSql for TransactionType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let value = match self {
            TransactionType::Debit => "d",
            TransactionType::Credit => "c",
        };

        Ok(value.into())
    }
}
