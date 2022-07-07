use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use actix::prelude::*;
use anyhow::Result;
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_files as fs;

use derive_more::Display;
use serde::{Deserialize, Serialize};

use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{Document};

use super::mongo;

#[derive(Deserialize, Serialize)]
pub struct TaskIndentifier {
    task_global_id: String,
}

pub struct MyActor {
    pub count: usize,
}

#[get("/task/{task_global_id}")]
pub async fn get_task(task_indentifier: Path<TaskIndentifier>) -> Json<String> {
    return Json(task_indentifier.into_inner().task_global_id);
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Ping(pub usize);

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;

        self.count
    }
}

use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};
use mongo::YuriPosts;

async fn gen_gallery() -> Json<Vec<YuriPosts>> {
    // >query mongodb for 'yuriPosts'
    // >find a mix of new posts and
    // most viewed posts
    // >limit to around 20 posts
    // >return json of them
    let collection = mongo::mongo_connect().await;

    let filter = doc! { "author": "Player01" };
    let find_options = FindOptions::builder().sort(doc! { "_id": 1 }).build();

    let mut cursor = collection.find(filter, find_options).await.expect("Failed to generate find cursor");
    let mut paths: Vec<YuriPosts> = Vec::new();
    let mut number = 0;

    while let Some(yuri_posts) = cursor.try_next().await.expect("Failed to iterate through cursor") {
        paths.push(yuri_posts);
        number += 1;
        if number > 20 {
            break;
        }
    }

    Json(paths)
}

#[get("/gallery_display")]
pub async fn gallery_display() -> Json<Vec<YuriPosts>> {
    return gen_gallery().await;
}
