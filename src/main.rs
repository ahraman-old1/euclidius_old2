use axum::{routing::get, Router};
use tokio::net::TcpListener;

use euclidius::routing::{page, root};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("File not found");

    let listener = TcpListener::bind("localhost:3000").await.unwrap();

    let router = Router::new()
        .route("/", get(root::show))
        .route("/:title", get(page::show));
    axum::serve(listener, router).await.unwrap();
}
