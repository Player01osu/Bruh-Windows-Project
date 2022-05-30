use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

async fn index(_req: HttpRequest) -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = if _req.path().eq("/") {
        "./files/index.html".parse().unwrap()
    } else if _req.path().eq("/about") {
        "./files/404.html".parse().unwrap()
    } else {
        "./files/404.html".parse().unwrap()
    };

    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(index))
        .route("/about", web::get().to(index))

        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

