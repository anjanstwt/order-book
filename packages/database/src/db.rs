use sea_orm::{Database, DatabaseConnection};

pub async fn connect(url: &str) -> DatabaseConnection {
    Database::connect(url).await.unwrap()
}
