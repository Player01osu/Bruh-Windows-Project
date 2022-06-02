use actix_files::NamedFile;
use actix_web::HttpRequest;
use actix_web::{get, http::Method, middleware, web, App, HttpServer, HttpResponse};
use actix_web::http::StatusCode;
use std::path::PathBuf;
use actix_files as fs;

// TODO: Properly implement 404 Error.
async fn not_found() -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = "./files/not_found.html".parse().unwrap();

    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| App::new()
        .service(fs::Files::new("/", "./files/").index_file("index.html"))
        .default_service(web::route().to(not_found)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
