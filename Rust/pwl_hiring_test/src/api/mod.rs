pub mod memory;
pub mod user;

use axum::{
    Router,
    extract::{FromRequest, rejection::JsonRejection},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing,
};
use mongodb::{Client, Database};
use user::ValidationError;

use crate::expect_env;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", routing::get(|| async { "Backend server is running" }))
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
        .nest("/users", users_router())
        .nest("/memory", memory_router(state.clone()))
        .with_state(state)
}

fn memory_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/save", routing::post(memory::save))
        .layer(middleware::from_fn_with_state(state, user::auth))
}

fn users_router() -> Router<AppState> {
    Router::new()
        .route("/register", routing::post(user::signup))
        .route("/login", routing::post(user::signin))
}

#[derive(Clone)]
pub struct AppState {
    conn: Client,
    jwt_secret: String,
}

impl AppState {
    pub fn db(&self) -> Database {
        self.conn.database("joao_xavier")
    }

    pub fn new(conn: Client, jwt_secret: String) -> Self {
        Self { conn, jwt_secret }
    }

    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
}

pub async fn db_conn() -> Client {
    let client = Client::with_uri_str(expect_env!("MONGODB_URI"))
        .await
        .expect("Failed to connect to mongo instance");

    tracing::debug!("MongoDB connected");

    client
}

pub type ApiResult<T> = Result<Json<T>, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Validation(#[from] ValidationError),
    #[error("Error during IO operation: {0}")]
    IO(#[from] mongodb::error::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let code = match &self {
            AppError::Validation(e) => {
                tracing::info!("{}", e);
                StatusCode::BAD_REQUEST
            }
            AppError::IO(e) => {
                tracing::error!("{}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let message = MessageResponse {
            message: self.to_string(),
        };

        (code, Json(message)).into_response()
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
        let err = ValidationError::InvalidJsonBodyParameters(rejection);
        AppError::Validation(err)
    }
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct Json<T>(T);

impl<T: serde::Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}
