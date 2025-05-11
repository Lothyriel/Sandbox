mod fragments;
mod routes;
mod services;

use std::net::Ipv4Addr;

use axum::{routing, Router};
use services::get_mongo_client;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or("debug,hyper=off".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = get_mongo_client()
        .await
        .expect("Expected to create mongo client")
        .database("cha");

    let app = Router::new()
        .route("/", routing::get(routes::index))
        .nest("/fragments", fragments::routes(db.to_owned()))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/favicon.ico", ServeFile::new("assets/favicon.ico"))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .fallback(routes::fallback_handler)
        .with_state(db);

    let address = (Ipv4Addr::UNSPECIFIED, 8080);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Expected to bind to network address");

    tracing::info!("Listening on {:?}", address);

    axum::serve(listener, app)
        .await
        .expect("Expected to start axum");
}
