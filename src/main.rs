mod api;

use std::ops::Index;
use actix_web::middleware::Logger;
use actix_web::{Responder, get, http::Method, middleware, web, App, HttpServer, HttpResponse, HttpRequest};
use actix_web::http::StatusCode;
use actix_files::NamedFile;
use actix_web::{
    error::ResponseError,
    post, put,
    web::Data,
    web::Json,
    web::Path,
};
use actix_rt; use actix::prelude::*;
use actix_files as fs;
use serde::{Deserialize, Serialize};

use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};

use api::task::{
    not_found,
    get_task,
    MyActor,
    Ping
};

#[derive(Serialize, Deserialize)]
struct Identity {
    id: u32,
    name: String,
}

#[get("/{id}/{name}/index.html")]
async fn index(index: Path<Identity>) -> impl Responder {
    format!("Hello {}! id:{}", index.name, index.id)
}

async fn static_index() -> impl Responder {
    let site = fs::Files::new("/", "./files/").index_file("index.html");

    Ok(site)
}

//#[get("/")]
//async fn static_handler() -> impl Responder {
//    fs::Files::new("/", "./files/").index_file("index.html")
//}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logging
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move|| {
        let logger = Logger::default();
        App::new()
        .wrap(logger)
        //.service(static_handler)
        .service(index)
        .service(get_task)
        .route("/", web::get().to(static_index))
        .service(fs::Files::new("/", "./files/").index_file("index.html"))
        .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
