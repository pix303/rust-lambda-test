use axum::{http::StatusCode, response::Json, routing::get, Router};
use lambda_http::{run, Error};
use serde::{Deserialize, Serialize};
use std::env::set_var;

#[derive(Serialize, Deserialize)]
struct WelcomeResponse {
    status: String,
    msg: String,
}
async fn welcome_handler() -> Json<WelcomeResponse> {
    Json(WelcomeResponse {
        status: StatusCode::OK.to_string(),
        msg: "Welcome to Rust Rest Api".to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let app = Router::new()
        .route("/", get(welcome_handler))
        .route("/welcome/", get(welcome_handler));
    run(app).await
}
