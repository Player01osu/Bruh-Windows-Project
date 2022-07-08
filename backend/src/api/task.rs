use std::fmt::format;

use actix_web::{HttpResponse, HttpResponseBuilder};
use actix_web::http::StatusCode;
use actix_web::web::JsonBody;
use actix_web::{
    get,
    post,
    web::Data,
    web::Json,
    web::Path,
};

use derive_more::Display;
use serde::{Deserialize, Serialize};

use mongodb::{Client, options::ClientOptions, Database};
use mongodb::bson::{Document};

use super::mongo::{self, MongodbCollection, MongodbDatabase};

#[derive(Deserialize, Serialize)]
pub struct TaskIndentifier {
    task_global_id: String,
}

use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};
use mongo::YuriPosts;

pub struct Gallery {
    show: Option<Json<Vec<YuriPosts>>>,
    search_filters: Option<String>,
    amount: u16
}

impl Gallery {
    pub fn new(amount: u16) -> Gallery {
        let generated = Gallery {
            show: None,
            search_filters: None,
            amount
        };
        generated
    }

    async fn gen_gallery(&mut self, database: Data<mongodb::Collection<YuriPosts>>) -> &mut Self {
        // >query mongodb for 'yuriPosts'
        // >find a mix of new posts and
        // most viewed posts
        // >limit to around 20 posts
        // >return json of them

        let database = MongodbDatabase::new(database);
        let filter = doc! { "author": "Player01" };
        let find_options = FindOptions::builder().sort(doc! { "_id": i32::from(1) }).build();

        let mut paths: Vec<YuriPosts> = Vec::new();
        database.find(filter, find_options, &mut paths, self.amount).await;

        if paths.is_empty() {
            self.show = None;
            return self;
        }

        self.show = Some(Json(paths));

        self
    }
}

#[get("/gallery_display")]
pub async fn gallery_display(database: Data<mongodb::Collection<YuriPosts>>) -> Json<Vec<YuriPosts>> {
    let mut generated = Gallery::new(20);
    generated.gen_gallery(database).await;

    return generated.show.unwrap();
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostImageRequest {
    title: String,
    time: u64,
    author: String,
    tags: Vec<String>,
}

// need YuriPosts to be Documents cuz yuriposts does not implement trait needed for isert
#[post("/post_image")]
pub async fn post_image(database: Data<mongodb::Collection<YuriPosts>>,
    request: Json<PostImageRequest>)
    -> HttpResponse {

    let path = format!("./assets/posts/{}.jpg", &request.title);

    let docs = YuriPosts {
        path,
        time: request.time.clone(),
        author: request.author.clone(),
        tags: request.tags.clone()
    };

    database.insert_one(docs, None).await.expect("Handle this error properly u lazy fuck");

    HttpResponse::Ok().body("yeet")
}
