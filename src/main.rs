mod api;
mod routing;

use std::path::PathBuf;
use actix_web::middleware::Logger;
use actix_web::web::Bytes;
use actix_web::{
    Responder,
    get,
    http::Method,
    middleware,
    web,
    App,
    HttpServer,
    HttpResponse,
    HttpRequest,
    Result,
};
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
use routing::routes;
use serde::{Deserialize, Serialize};

use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};
use once_cell::sync::*;
use dashmap::DashMap;
use compact_str::*;
use ahash::RandomState;


use api::task::{
    not_found,
    get_task,
    MyActor,
    Ping
};

use routes::*;

#[derive(Serialize, Deserialize)]
struct Identity {
    id: u32,
    name: String,
}

#[get("/{id}/{name}/index.html")]
async fn index(index: Path<Identity>) -> impl Responder {
    format!("Hello {}! id:{}", index.name, index.id)
}

fn main() -> anyhow::Result<()> {
    init()
}


#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    // Logging
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    //init().ok();
    //let (static_served_from, static_mount_path): (String, PathBuf) = get_static().unwrap();

    //let ff: PathBuf = "./static/assets/".into();
    // Insert 404 Page Not Found
    let not_found_page = StaticFile {
        bytes: Bytes::from(include_bytes!("./static/404.html").to_vec()),
        path: PathBuf::from("static/404.html"),
    };
    let route_handle = RouteHandle { response: not_found_page };
    ROUTEMAP.insert("{{404}}".into(), route_handle);


    HttpServer::new(move|| {
        let logger = Logger::default();
        let app_instance = App::new()
            .wrap(logger)
            //.service(static_handler)
            .service(index)
            .service(get_task)
            .service({
                //let static_serve = actix_files::Files::new(&static_served_from, &static_mount_path);
                let static_serve = actix_files::Files::new("/assets", "static/assets");
                static_serve
                })
            //.service(fs::Files::new("/", "./files/").index_file("index.html"))
            .default_service(web::route().to(router));

        app_instance
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
