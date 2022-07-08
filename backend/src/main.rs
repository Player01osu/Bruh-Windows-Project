mod api;
mod routing;

use std::path::PathBuf;
use actix_web::middleware::{Logger, self};
use actix_web::web::Bytes;
use actix_web::{
    web,
    App,
    HttpServer,
};
use actix_web::{
    web::Data,
};
use routing::routes;
use serde::{Deserialize, Serialize};

use api::task::{
    gallery_display,
    post_image
};

use routes::*;
use api::mongo::{MongodbCollection, MongodbDatabase};

fn main() -> anyhow::Result<()> {
    init()
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    // Logging
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // Insert 404 Page Not Found
    let not_found_page = StaticFile {
        bytes: Bytes::from(include_bytes!("./static/404.html").to_vec()),
        path: PathBuf::from("static/404.html"),
    };
    let route_handle = RouteHandle { response: not_found_page };
    ROUTEMAP.insert("{{404}}".into(), route_handle);

    let database = MongodbDatabase::mongo_connect().await;

    HttpServer::new(move|| {
        let database_data = Data::new(database.clone());
        let logger = Logger::default();

        let app_instance = App::new()
            .wrap(logger)
            .wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Trim))
            .app_data(database_data)
            .service(
                web::scope("/api")
                    .service(gallery_display)
                    .service(post_image)
                )
            .service(actix_files::Files::new("/assets", "static/assets"))
            .default_service(web::route().to(router));

        app_instance
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
