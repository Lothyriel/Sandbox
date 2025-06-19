pub mod poi;
pub mod route;

use axum::{
    Router,
    extract::{FromRequest, rejection::JsonRejection},
    http::StatusCode,
    response::IntoResponse,
    routing,
};
use mongodb::{Client, Database};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", routing::get(|| async { "healthy" }))
        .nest("/api", api_router(state))
        .fallback(not_found)
}

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        MessageResponse::new("The requested resource could not be found."),
    )
}

fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/poi", routing::get(poi::get))
        .route("/poi", routing::post(poi::add))
        .route("/poi/{id}", routing::put(poi::put))
        .route("/routes", routing::post(route::calculate))
        .route("/routes", routing::get(route::suggested))
        .with_state(state)
}

#[derive(Clone)]
pub struct AppState {
    conn: mongodb::Client,
    http_client: reqwest::Client,
    openroute_key: String,
}

impl AppState {
    pub fn db(&self) -> Database {
        self.conn.database("hackathorion_api")
    }

    pub fn new(conn: Client, openroute_key: String) -> Self {
        Self {
            conn,
            openroute_key,
            http_client: reqwest::Client::new(),
        }
    }
}

pub async fn db_conn(uri: &str) -> Client {
    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect to mongo instance");

    tracing::debug!("MongoDB connected");

    client
}

pub struct AppError(anyhow::Error);

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError(value)
    }
}

pub type ApiResult<T> = Result<Json<T>, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let message = MessageResponse {
            message: self.0.to_string(),
        };

        (StatusCode::INTERNAL_SERVER_ERROR, Json(message)).into_response()
    }
}

#[derive(serde::Serialize)]
pub struct MessageResponse {
    message: String,
}

impl MessageResponse {
    pub fn new(message: impl Into<String>) -> Json<Self> {
        let msg = Self {
            message: message.into(),
        };

        Json(msg)
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        AppError(anyhow::anyhow!("{}", rejection.body_text()))
    }
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct Json<T>(T);

impl<T: serde::Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}
