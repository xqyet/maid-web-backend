use axum::{
    routing::post,
    Json, Router,
};
use hyper::Server;
use maid_lang::run;
use serde::Deserialize;
use std::{net::SocketAddr};
use tower_http::services::ServeDir;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{self, Read, Write, BufReader};
use std::os::windows::io::AsRawHandle;
use gag::BufferRedirect;

#[derive(Deserialize)]
struct RunRequest {
    code: String,
}

#[tokio::main]
async fn main() {
    // Serve static files from ./static
    let serve_dir = ServeDir::new("static").not_found_service(ServeDir::new("static"));

    let app = Router::new()
        .route("/run", post(run_code))
        .fallback_service(serve_dir); // serve index.html and assets

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 MaidLang running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn run_code(Json(payload): Json<RunRequest>) -> String {
    use std::io::BufReader;

    // Capture stdout
    let stdout_redirect = BufferRedirect::stdout().unwrap();
    let mut reader = BufReader::new(stdout_redirect);

    // Run MaidLang code
    let result = run("<stdin>", Some(payload.code));

    // Read from redirected stdout
    let mut captured = String::new();
    reader.read_to_string(&mut captured).unwrap();

    match result {
        Some(err) => format!("❌ {}\n{}", err, captured),
        None => format!("Code executed successfully\n{}", captured),
    }
}