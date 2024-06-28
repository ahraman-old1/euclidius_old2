use axum::response::{IntoResponse, Redirect};

pub async fn show() -> impl IntoResponse {
    Redirect::to("/main")
}
