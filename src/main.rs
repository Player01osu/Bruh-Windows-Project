use actix_files::NamedFile;
use actix_web::{HttpResponse, HttpRequest, Result, guard, get};
use actix_web::http::StatusCode;
use std::path::PathBuf;

async fn index(_req: HttpRequest) -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = "./files/index.html".parse().unwrap();

    Ok(NamedFile::open(path)?)
}

async fn test(_req: HttpRequest) -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = "./files/test.html".parse().unwrap();

    Ok(NamedFile::open(path)?)
}

async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            .service(web::resource("/test").to(test))
            .default_service(web::route().to(not_found))
        })
        .run()
        .await
}

