use common::mongodb::structs::{Resolution, Source, YuriPosts};

use actix_web::{web::Data, web::Json};
use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PostImageRequest {
    title: String,
    author: String,
    op: String,
    source: Source,
    resolution: Resolution,
    time: u64,
    tags: Option<Vec<String>>,
    file_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LikeImageRequest {
    oid: String,
}
#[derive(Deserialize, Serialize)]
pub struct TaskIndentifier {
    task_global_id: String,
}

use crate::database::mongo::MongodbDatabase;
#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteImageRequest {
    pub oid: String,
}

#[derive(Serialize, Deserialize)]
pub struct ViewPostsPath {
    pub page_number: u16,
    pub sort: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    #[serde(default)]
    pub query: String,
}

pub struct Gallery {
    pub show: Option<Json<Vec<Document>>>,
    search_filters: Option<Vec<String>>,
    amount: u16,
}

impl Gallery {
    pub fn new(amount: u16) -> Gallery {
        let generated = Gallery {
            show: None,
            search_filters: None,
            amount,
        };
        generated
    }

    pub async fn gen_gallery(
        &mut self,
        database: Data<mongodb::Collection<YuriPosts>>,
        sort: &String,
        query: &String,
    ) -> &mut Self {
        let database = MongodbDatabase::new(database);
        let sort = match sort.as_str() {
            "new" => String::from("time"),
            "top" => String::from("stats.likes"),
            "views" => String::from("stats.views"),
            _ => String::from("time"),
        };
        let skip_amount = u32::from(self.amount - 10);
        let limit = i64::from(self.amount);

        let filter = Some(doc! {
            "$or": [
                { "title": { "$regex": query, "$options": "i" } },
                { "author": { "$regex": query, "$options": "i" } },
                { "op": { "$regex": query, "$options": "i" } },
                { "tags": { "$regex": query, "$options": "i" } },
                { "source.material": { "$regex": query, "$options": "i" } }
            ]
        });
        let find_options = Some(
            FindOptions::builder()
                .skip(u64::from(skip_amount))
                .limit(limit)
                .sort(doc! {sort: -1, "time": -1})
                .build(),
        );

        let paths = database.find(filter, find_options).await;

        match !paths.is_empty() {
            true => {
                self.show = Some(Json(paths));
                self
            }
            false => {
                self.show = None;
                self
            }
        }
    }
}
