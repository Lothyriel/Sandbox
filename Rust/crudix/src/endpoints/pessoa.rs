use actix_web::{
    http::StatusCode,
    web::{self, Json},
    Responder, Result, Scope,
};
use anyhow::Error;
use futures::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use serde::Deserialize;

use crate::{domain::pessoa::Pessoa, endpoints::extensions::endpoint_ext::IntoHttpError};

pub fn configure() -> Scope {
    web::scope("/pessoa")
        .service(web::resource("/add").route(web::post()).to(add))
        .service(web::resource("/get").route(web::get().to(get_all)))
        .service(web::resource("/get/{id}").route(web::get().to(get)))
}

#[derive(Deserialize)]
struct PathInfo(String);

async fn get_all() -> Result<impl Responder, actix_web::Error> {
    let database = connect_db()
        .await
        .http_internal_error("Não conectou no banco...")?;

    let collection = database.collection::<Pessoa>("pessoa");

    let cursor = collection
        .find(None, None)
        .await
        .http_internal_error("nao pude mitar")?;

    let result: Vec<Pessoa> = cursor
        .try_collect()
        .await
        .http_internal_error("nao pude mitar 2")?;

    Ok(Json(result))
}

async fn get(pessoa: web::Path<PathInfo>) -> Result<impl Responder, actix_web::Error> {
    let database = connect_db()
        .await
        .http_internal_error("Não conectou no banco...")?;

    let collection = database.collection::<Pessoa>("pessoa");

    let filter = doc! { "_id": pessoa.0.to_owned() };

    let data = collection
        .find_one(filter, None)
        .await
        .http_internal_error("Não consegui buscar no banco...")?;

    let result = data
        .ok_or_else(|| Error::msg("Pessoa não encontrada"))
        .http_error("Não encontrou a pessoa no banco...", StatusCode::NOT_FOUND)?;

    Ok(Json(result))
}

async fn add(pessoa: web::Json<Pessoa>) -> Result<impl Responder, actix_web::Error> {
    let database = connect_db()
        .await
        .http_internal_error("Banco não conectou...")?;

    let data = database
        .collection::<Pessoa>("pessoa")
        .insert_one(pessoa.0, None)
        .await
        .http_internal_error("Não inseriu...")?;

    Ok(Json(data))
}

async fn connect_db() -> Result<Database, Error> {
    let client_options = ClientOptions::parse("mongodb://localhost:8000").await?;

    let client = Client::with_options(client_options)?;

    Ok(client.database("admin"))
}
