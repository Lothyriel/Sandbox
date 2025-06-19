use std::net::Ipv4Addr;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api::{AppState, db_conn, router};

mod api;
mod common;
mod infra;
mod models;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(
            "debug,hyper=off,rustls=error,tungstenite=error,hickory=off",
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_line_number(true)
                .with_file(true),
        )
        .init();

    std::panic::set_hook(Box::new(|panic_info| {
        tracing::error!("FATAL Error: {:?}", panic_info);
    }));

    dotenvy::dotenv().ok();

    let db = db_conn().await;

    let jwt_secret = expect_env!("JWT_SECRET");

    let state = AppState::new(db, jwt_secret);

    let cors = tower_http::cors::CorsLayer::permissive();

    let app = router(state).layer(cors);

    let address = (Ipv4Addr::UNSPECIFIED, 3000);
    tracing::debug!("Server running on {:?}", address);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind to network address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server")
}
