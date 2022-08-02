mod api;
mod routing;

use actix_cors::Cors;
use actix_web::middleware::{self, Logger};
use actix_web::web::Bytes;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use routing::routes;
use std::path::PathBuf;

use api::task::{delete_post, like_post, post_image, unlike_post, view_posts};

use api::mongo::MongodbDatabase;
use routes::*;

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
    let route_handle = RouteHandle {
        response: not_found_page,
    };
    ROUTEMAP.insert("{{404}}".into(), route_handle);

    let database = MongodbDatabase::mongo_connect().await;

    HttpServer::new(move || {
        let database_data = Data::new(database.clone());
        let logger = Logger::default();
        let cors = Cors::permissive(); // FIXME: uhhhhhhhh, change this

        let app_instance = App::new()
            .wrap(cors)
            .wrap(logger)
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .app_data(database_data)
            // Load assets
            .service(actix_files::Files::new("/assets", "static/assets"))
            .service(
                web::scope("/api")
                    .service(view_posts)
                    .service(post_image)
                    .service(delete_post)
                    .service(like_post)
                    .service(unlike_post),
            )
            .default_service(web::route().to(router));

        app_instance
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}
