use actix_web::web::Path;
use actix_web::web::Payload;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::StreamExt;

use log::info;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

mod config;

async fn upload_handler(
    path: Path<(String,)>,
    mut payload: Payload,
) -> Result<HttpResponse, Error> {
    let filename = path.into_inner().0;
    let target_dir = &config::CONFIG.target_dir;

    log::info!("Uploading file: {}", filename);

    // Ensure the target directory exists
    if !std::path::Path::new(target_dir).exists() {
        fs::create_dir_all(target_dir)?;
    }

    let filepath = PathBuf::from(target_dir).join(filename);
    let mut file = File::create(filepath)?;

    let mut file_size = 0;
    while let Some(chunk) = payload.next().await {
        let data = chunk?;
        file.write_all(&data)?;
        file_size += data.len() as u64;
    }

    let message = format!(
        "File uploaded successfully. {} bytes written to disk.",
        file_size
    );

    log::info!("{}", message);

    Ok(HttpResponse::Ok().content_type("text/plain").body(message))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_level = config::CONFIG.log_level.clone();
    let version = env!("CARGO_PKG_VERSION");
    let server_port = config::CONFIG.server_port;

    // Create logger

    let level_filter = match log_level.to_lowercase().as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "warn" => log::LevelFilter::Warn,
        "warning" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Info,
    };

    env_logger::Builder::from_default_env()
        .filter(None, level_filter)
        .init();

    // Start HTTP2AMCP server

    info!("Starting fileup {} server on port {}", version, server_port);

    HttpServer::new(|| App::new().route("/upload/{filename}", web::post().to(upload_handler)))
        .bind(format!("0.0.0.0:{}", server_port))?
        .run()
        .await
}
