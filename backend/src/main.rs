mod api;
mod database;
mod routing;

use crate::database::mongo::MongodbDatabase;
use actix_cors::Cors;
use actix_web::middleware::{self, Logger};
use actix_web::web::Bytes;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use api::comment::{post_comment::post_comment, view_comments::view_post_comments};
use api::post::{
    delete_post::delete_post,
    like_post::{like_post, unlike_post},
    upload_post::post_image,
    view_post::view_posts,
};
use common::mongodb::structs::YuriPosts;
use routing::routes;
use std::path::PathBuf;

use routes::*;

use common::mongodb::structs::CommentSection;

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

    let (posts_collection, comments_collection) = futures::join!(
        MongodbDatabase::mongo_connect::<YuriPosts>("yuriPosts"),
        MongodbDatabase::mongo_connect::<CommentSection>("yuriComments"),
    );

    HttpServer::new(move || {
        let posts_collection = Data::new(posts_collection.clone());
        let comments_collection = Data::new(comments_collection.clone());
        let logger = Logger::default();
        let cors = Cors::permissive(); // FIXME: uhhhhhhhh, change this

        let app_instance = App::new()
            .wrap(cors)
            .wrap(logger)
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .app_data(posts_collection)
            .app_data(comments_collection)
            // Load assets
            .service(actix_files::Files::new("/assets", "static/assets"))
            .service(
                web::scope("/api")
                    .service(view_posts)
                    .service(post_image)
                    .service(delete_post)
                    .service(like_post)
                    .service(unlike_post)
                    .service(view_post_comments)
                    .service(post_comment),
            )
            .default_service(web::route().to(router));

        app_instance
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}
