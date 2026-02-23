use std::env;

use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use server::{bootstrap::build_dependencies, database::postgres_connector::connect, server::build_server};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    // Env variables
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not set in .env file");
    let port = env::var("PORT")
        .expect("PORT is not set in .env file")
        .parse::<u16>()
        .unwrap();

    // Database stuff
    let db: DatabaseConnection = connect(&db_url).await;
    let client = redis::Client::open(redis_url).unwrap();

    // Building Dependencies
    let dependencies = build_dependencies(db, client);

    build_server(dependencies, port).await
}
