use actix_files::NamedFile;
use actix_web::{get, http::Method, middleware, web, App, HttpServer, HttpResponse};
use actix_web::http::StatusCode;
use std::path::PathBuf;

async fn index() -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = "./files/index.html".parse().unwrap();

    Ok(NamedFile::open(path)?)
}

async fn test() -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = "./files/test.html".parse().unwrap();

    Ok(NamedFile::open(path)?)
}

async fn not_found() -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = "./files/404.html".parse().unwrap();

    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .service(web::resource("/").to(index))
            .service(web::resource("/testI").to(test))
            .default_service(web::route().to(not_found))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

