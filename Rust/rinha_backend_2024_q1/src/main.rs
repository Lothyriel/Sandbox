mod db;
mod handlers;
mod models;
mod routes;

const SEED_DATA: [(u32, u32); 5] = [
    (1, 100000),
    (2, 80000),
    (3, 1000000),
    (4, 10000000),
    (5, 500000),
];

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    std::fs::create_dir_all("db")?;

    let pool = async_sqlite::PoolBuilder::new()
        .journal_mode(async_sqlite::JournalMode::Wal)
        .path("db/rinha.db")
        .open()
        .await?;

    pool.conn(|conn| {
        db::grant_database_tables(conn)?;
        db::seed_data(conn, &SEED_DATA)?;

        Ok(())
    })
    .await?;

    let app = axum::Router::new()
        .nest("/clientes/:id", routes::router())
        .with_state(AppState::new(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    db_pool: async_sqlite::Pool,
}

impl AppState {
    fn new(db_pool: async_sqlite::Pool) -> Self {
        Self { db_pool }
    }
}
