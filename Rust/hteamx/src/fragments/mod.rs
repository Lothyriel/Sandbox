use axum::{extract::State, response::IntoResponse, routing, Router};
use axum_extra::extract::{cookie::Cookie, CookieJar, Form};
use mongodb::Database;

use crate::{
    routes::{ResponseError, COOKIE_NAME},
    services::ParticipantsRepository,
};

pub fn routes(db: Database) -> Router<Database> {
    Router::new()
        .route("/confirmation", routing::post(set_confirmation))
        .route("/add", routing::get(add_participant))
        .with_state(db)
}

async fn add_participant() -> AddTemplate {
    AddTemplate
}

async fn set_confirmation(
    jar: CookieJar,
    State(db): State<Database>,
    Form(form): Form<ConfirmationInfoForm>,
) -> Result<impl IntoResponse, ResponseError> {
    let cookie = Cookie::build((COOKIE_NAME, form.name.trim().to_owned()))
        .http_only(true)
        .path("/")
        .max_age(time::Duration::days(30));

    ParticipantsRepository::new(&db).upsert(form).await?;

    Ok((jar.add(cookie), ConfirmedTemplate))
}

#[derive(Debug, serde::Deserialize)]
pub struct ConfirmationInfoForm {
    pub name: String,
    #[serde(default)]
    pub escorts: Vec<String>,
}

#[derive(askama::Template)]
#[template(path = "confirm.html")]
pub struct ConfirmTemplate {
    pub name: Option<String>,
    pub escorts: Vec<String>,
    pub confirmed: bool,
}

#[derive(askama::Template)]
#[template(path = "add.html")]
pub struct AddTemplate;

#[derive(askama::Template)]
#[template(path = "confirmed.html")]
pub struct ConfirmedTemplate;
