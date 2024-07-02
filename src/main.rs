use axum::{routing::get, Router};
use euclidius_migration::DatabaseAddress;
use sea_orm::DbErr;
use tokio::net::TcpListener;

use euclidius::routing::{page, root};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("File not found");

    init_database_connection().await.unwrap();

    let listener = TcpListener::bind("localhost:3000").await.unwrap();

    let router = Router::new()
        .route("/", get(root::show))
        .route("/:title", get(page::show));
    axum::serve(listener, router).await.unwrap();
}

async fn init_database_connection() -> Result<(), DbErr> {
    let address = DatabaseAddress::from_env().unwrap();
    euclidius_migration::run(address).await?;

    Ok(())
}
