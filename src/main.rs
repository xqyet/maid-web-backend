use axum::{
    routing::{get, post},
    Json, Router,
};
use hyper::Server;
use maid_lang::run;
use serde::Deserialize;
use std::{net::SocketAddr};
use tower_http::services::ServeDir;
use std::io::Cursor;
use std::sync::Mutex;
use std::io::Read;

#[derive(Deserialize)]
struct RunRequest {
    code: String,
}

#[tokio::main]
async fn main() {
    // Serve static files from the /static directory
    let serve_dir = ServeDir::new("static").not_found_service(ServeDir::new("static")); // fallback to index.html

    let app = Router::new()
        .route("/run", post(run_code))
        .fallback_service(serve_dir); // serves index.html and others

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 MaidLang running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn run_code(Json(payload): Json<RunRequest>) -> String {
    match run("<stdin>", Some(payload.code)) {
        Some(error) => format!("❌ {}", error),
        None => String::from("✅ Code executed successfully"),
    }
}
