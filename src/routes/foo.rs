use axum::http::StatusCode;

pub async fn post() -> StatusCode {
    println!("hey!");
    StatusCode::OK
}