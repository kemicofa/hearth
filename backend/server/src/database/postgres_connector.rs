use sea_orm::{Database, DatabaseConnection};

pub async fn connect(database_url: &String) -> DatabaseConnection {
    Database::connect(database_url).await.expect("Failed to connect to postgres database")
}