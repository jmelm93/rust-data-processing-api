use actix_web::{get};

#[get("")]
async fn index() -> String {
    "Having a jolly good ole time with Rust.".to_string()
}

