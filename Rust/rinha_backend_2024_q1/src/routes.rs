use anyhow::anyhow;
use async_sqlite::rusqlite::Error;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::{handlers, models::*, AppState};

#[derive(Debug)]
pub enum ErrorResponse {
    Transient(anyhow::Error),
    ClientNotFound(ClientId),
    NotEnoughLimit,
    InvalidDescription,
}

impl From<anyhow::Error> for ErrorResponse {
    fn from(value: anyhow::Error) -> Self {
        ErrorResponse::Transient(value)
    }
}

impl From<Error> for ErrorResponse {
    fn from(value: Error) -> Self {
        ErrorResponse::Transient(anyhow!(value))
    }
}

impl From<async_sqlite::Error> for ErrorResponse {
    fn from(value: async_sqlite::Error) -> Self {
        ErrorResponse::Transient(anyhow!(value))
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let error = match self {
            ErrorResponse::ClientNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Client with id {{{}}} not found", id),
            ),
            ErrorResponse::NotEnoughLimit | ErrorResponse::InvalidDescription => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Not enough limit to complete this transaction".to_owned(),
            ),
            ErrorResponse::Transient(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        error.into_response()
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/transacoes", post(add_transaction))
        .route("/extrato", get(get_extract))
}

async fn add_transaction(
    State(state): State<AppState>,
    Path(client_id): Path<ClientId>,
    Json(request): Json<TransactionRequest>,
) -> Result<Json<TransactionResponse>, ErrorResponse> {
    state
        .db_pool
        .conn_mut(move |conn| Ok(handlers::add_transaction(conn, client_id, request).map(Json)))
        .await?
}

async fn get_extract(
    State(state): State<AppState>,
    Path(client_id): Path<ClientId>,
) -> Result<Json<ExtractResponse>, ErrorResponse> {
    state
        .db_pool
        .conn(move |conn| Ok(handlers::get_extract(conn, client_id).map(Json)))
        .await?
}
