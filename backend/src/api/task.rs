use actix_web::{
    get,
    web::Data,
    web::Json,
    web::Path,
};
use actix::prelude::*;

use derive_more::Display;
use serde::{Deserialize, Serialize};

use mongodb::{Client, options::ClientOptions, Database};
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

        let filter = doc! { "author": "Player01" };
        let find_options = FindOptions::builder().sort(doc! { "_id": 1 }).build();

        let mut cursor = database.find(filter, find_options).await.expect("Failed to generate find cursor");
        let mut paths: Vec<YuriPosts> = Vec::new();
        let mut number = 0;

        while let Some(yuri_posts) = cursor.try_next().await.expect("Failed to iterate through cursor") {
            println!("path: {}", yuri_posts.path);
            paths.push(yuri_posts);
            number += 1;
            if number > self.amount {
                break;
            }
        }

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
