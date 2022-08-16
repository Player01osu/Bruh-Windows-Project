mod api;
mod database;

use crate::database::mongo::MongodbDatabase;
use actix_cors::Cors;
use actix_web::middleware::{self, Logger};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_web_lab::web::spa;
use api::{comment::{post_comment::post_comment, view_comments::view_post_comments}, post::{
    delete_post::delete_post,
    like_post::{like_post, unlike_post},
    upload_post::post_image,
    view_post::view_posts,
}, user::{get_user::get_user, generate_user::generate_user}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logging
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let database = MongodbDatabase::mongo_connect().await;

    HttpServer::new(move || {
        let logger = Logger::default();
        let database = database.clone();
        let database = Data::new(database);
        let cors = Cors::permissive(); // FIXME: uhhhhhhhh, change this

        let app_instance = App::new()
            .wrap(cors)
            .wrap(logger)
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .app_data(database)
            .service(
                web::scope("/api")
                    .service(view_posts)
                    .service(post_image)
                    .service(delete_post)
                    .service(like_post)
                    .service(unlike_post)
                    .service(post_comment)
                    .service(view_post_comments)
                    .service(
                        web::scope("/user")
                            .service(get_user)
                            .service(generate_user)
                        ),
            )
            .service(
                spa()
                .index_file("./dist/index.html")
                .static_resources_mount("/")
                .static_resources_location("./dist")
                .finish()
            );

        app_instance
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}
