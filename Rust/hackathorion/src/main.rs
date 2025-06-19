use std::net::Ipv4Addr;

use app::{AppState, db_conn, route::Waypoint, router};
use futures::TryStreamExt;
use infra::export_maps_location;
use mongodb::{
    Client, Collection,
    bson::{self, doc},
    options::ClientOptions,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod infra;

macro_rules! expect_env {
    ($var_name:expr) => {
        std::env::var($var_name).expect(concat!("env missing: ", $var_name))
    };
}

async fn migrate() -> mongodb::error::Result<()> {
    // Connect to MongoDB
    let client_uri = "mongodb+srv://hackathorion:XVYwdxE6fQlPqh3q@cluster.lykhh33.mongodb.net/?retryWrites=true&w=majority&appName=cluster";
    let client_options = ClientOptions::parse(client_uri).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("hackathorion_api");
    let collection: Collection<bson::Document> = db.collection("poi");

    let mut cursor = collection.find(doc! {}).await?;

    while let Some(doc) = cursor.try_next().await? {
        let a = doc.get("coords").unwrap().as_array().unwrap();

        let b = a.get(0).unwrap().as_f64().unwrap();
        let c = a.get(1).unwrap().as_f64().unwrap();

        if let Some(id) = doc.get("_id") {
            let update = doc! {
                "$set": {
                    "google_maps_route": export_maps_location(&Waypoint{lat: b, lon: c})
                }
            };

            collection
                .update_one(doc! { "_id": id.clone() }, update)
                .await?;
        }
    }

    println!("All documents updated.");
    Ok(())
}

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

    dotenvy::dotenv().ok();

    let db = db_conn(&expect_env!("MONGODB_URI")).await;

    let state = AppState::new(db, expect_env!("OPEN_ROUTE_API_KEY"));

    let cors = tower_http::cors::CorsLayer::permissive();

    let app = router(state).layer(cors);

    let address = (Ipv4Addr::UNSPECIFIED, 8080);
    tracing::debug!("Server running on {:?}", address);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind to network address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server")
}
