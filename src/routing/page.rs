use axum::{extract::Path, response::IntoResponse};

pub async fn show(Path(title): Path<String>) -> impl IntoResponse {
    format!("looking for page '{title}'")
}
