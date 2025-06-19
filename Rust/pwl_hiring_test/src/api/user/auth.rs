use crate::{
    api::{AppState, Json},
    infra::dto::ReadHashedUser,
};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::Error};
use mongodb::bson::{oid::ObjectId, serde_helpers::serialize_object_id_as_hex_string};
use serde_json::json;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct UserClaims {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub username: String,
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Auth token not found on the request")]
    TokenNotPresent,
    #[error("Invalid token: ({0})")]
    JwtValidation(#[from] jsonwebtoken::errors::Error),
}

pub async fn generate_token(user: ReadHashedUser, secret: &str) -> String {
    let data = UserClaims {
        username: user.username,
        id: user.id,
    };

    let now = Utc::now();

    let claims = JwtPayload {
        data,
        iss: ISSUER.to_string(),
        exp: (now + Duration::hours(1)).timestamp(),
        nbf: now.timestamp(),
    };

    let result = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    );

    result.expect("Failed to encode JWT")
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    cookies: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AuthError> {
    let cookie = cookies.get("token").ok_or(AuthError::TokenNotPresent)?;

    let claims = decode(state.jwt_secret(), cookie.value()).await?;

    req.extensions_mut().insert(claims.data);

    Ok(next.run(req).await)
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({"error": self.to_string() }));

        (StatusCode::UNAUTHORIZED, body).into_response()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct JwtPayload {
    data: UserClaims,
    exp: i64,
    nbf: i64,
    iss: String,
}

const ISSUER: &str = "joao.xavier.api";

async fn decode(secret: &str, token: &str) -> Result<JwtPayload, Error> {
    let key = DecodingKey::from_secret(secret.as_bytes());

    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);

    validation.set_issuer(&[ISSUER]);

    let claims = jsonwebtoken::decode(token, &key, &validation)?.claims;

    Ok(claims)
}
