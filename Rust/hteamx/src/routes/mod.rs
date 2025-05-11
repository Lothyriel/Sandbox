use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use mongodb::{error::Error, Database};

use crate::{fragments::ConfirmTemplate, services::ParticipantsRepository};

pub const COOKIE_NAME: &str = "PARTICIPANT_NAME";

const NOT_FOUND_RESPONSE: (StatusCode, &str) =
    (StatusCode::NOT_FOUND, "this resource doesn't exist");

#[derive(askama::Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    confirm: ConfirmTemplate,
}

pub async fn index(
    jar: CookieJar,
    State(db): State<Database>,
) -> Result<IndexTemplate, ResponseError> {
    let conf_template = if let Some(c) = jar.get(COOKIE_NAME) {
        ParticipantsRepository::new(&db).get_info(c.value()).await?
    } else {
        None
    };

    Ok(IndexTemplate {
        confirm: ConfirmTemplate {
            confirmed: conf_template.is_some(),
            name: conf_template.as_ref().map(|c| c.name.to_owned()),
            escorts: conf_template.and_then(|c| c.escorts).unwrap_or_default(),
        },
    })
}

pub async fn fallback_handler() -> (StatusCode, &'static str) {
    NOT_FOUND_RESPONSE
}

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("Database error: {0}")]
    Database(#[from] Error),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            ResponseError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (code, self.to_string()).into_response()
    }
}
