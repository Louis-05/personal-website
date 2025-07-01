
use axum::response::IntoResponse;


pub async fn get_style() -> impl IntoResponse {
    include_str!("../../templates/style.css")
}
