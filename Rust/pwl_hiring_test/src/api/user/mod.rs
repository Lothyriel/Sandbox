mod auth;

use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::IntoResponse;
use axum_extra::extract::cookie::{Cookie, SameSite};
use bcrypt::verify;

pub use auth::UserClaims;
pub use auth::auth_middleware as auth;

use crate::infra::{
    dto::{UserSignin, UserSignup},
    repositories::UserRepository,
};

use super::{ApiResult, AppError, AppState, Json, MessageResponse};

#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("Username already exists")]
    UsernameAlreadyTaken,
    #[error("Invalid username or password")]
    InvalidCredentials,
    #[error("Password in invalid format")]
    InvalidPasswordFormat(#[from] bcrypt::BcryptError),
    #[error("Invalid json body parameters: {0}")]
    InvalidJsonBodyParameters(JsonRejection),
}

pub async fn signup(
    State(state): State<AppState>,
    Json(user): Json<UserSignup>,
) -> ApiResult<MessageResponse> {
    let result = state.db().find_user(&user.username).await?;

    if result.is_some() {
        return bail(ValidationError::UsernameAlreadyTaken);
    }

    state.db().register(user).await?;

    Ok(MessageResponse::new("User registered successfully"))
}

pub async fn signin(
    State(state): State<AppState>,
    Json(user): Json<UserSignin>,
) -> Result<impl IntoResponse, AppError> {
    let result = state.db().find_user(&user.username).await?;

    let Some(db_user) = result else {
        return bail(ValidationError::InvalidCredentials);
    };

    let authenticated =
        verify(user.password, &db_user.password).map_err(ValidationError::InvalidPasswordFormat)?;

    if !authenticated {
        return bail(ValidationError::InvalidCredentials);
    }

    let token = auth::generate_token(db_user, &state.jwt_secret).await;

    let cookie = Cookie::build(("token", token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .build();

    let mut headers = HeaderMap::new();

    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok((StatusCode::OK, headers).into_response())
}

fn bail<T>(err: ValidationError) -> Result<T, AppError> {
    Err(AppError::Validation(err))
}
