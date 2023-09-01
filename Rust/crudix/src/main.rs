use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use env_logger;
use std::env;

mod domain;
mod endpoints;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .service(endpoints::pessoa::configure())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
