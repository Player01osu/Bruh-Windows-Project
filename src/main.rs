use actix_files::NamedFile;
use actix_web::{HttpResponse, HttpRequest, Result, guard, get};
use actix_web::http::StatusCode;
use std::path::PathBuf;

async fn index(_req: HttpRequest) -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = "./files/index.html".parse().unwrap();

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
        .route("/", web::get().to(index))

        .service(
            web::resource("/user/{name}")
                .name("user_detail")
                .guard(guard::Header("content-type", "application/json"))
                .route(web::get().to(HttpResponse::Ok))
                .route(web::put().to(HttpResponse::Ok)),
        )
        .default_service(web::route().to(not_found))
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

