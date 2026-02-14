use sea_orm_migration::prelude::*;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    cli::run_cli(migration::Migrator).await;
}
