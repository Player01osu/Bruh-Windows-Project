use actix_web::http::StatusCode;
use actix_web::web::JsonBody;
use actix_web::{delete, get, post, put, web::Data, web::Json, web::Path};
use actix_web::{HttpResponse, HttpResponseBuilder};

use serde::{Deserialize, Serialize};

use mongodb::bson::Document;

use super::mongo::{self, MongodbCollection, MongodbDatabase};

#[derive(Deserialize, Serialize)]
pub struct TaskIndentifier {
    task_global_id: String,
}

use common::mongodb::structs::{YuriPosts, PostStats, Comment};
use mongodb::{bson::doc, options::FindOptions};

pub struct Gallery {
    show: Option<Json<Vec<Document>>>,
    search_filters: Option<String>,
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

    async fn gen_gallery(&mut self, database: Data<mongodb::Collection<YuriPosts>>, sort: String) -> &mut Self {
        // >query mongodb for 'yuriPosts'

        let database = MongodbDatabase::new(database);
        //let filter = doc! { "op": "Player01" };

        let find_options = match sort.as_str() {
            "new" => FindOptions::builder()
                .limit(i64::from(self.amount))
                .sort( doc! {"time":-1})
                .build(),
            "top" => FindOptions::builder()
                .limit(i64::from(self.amount))
                .sort( doc! {"stats.likes":-1})
                .build(),
            "views" => FindOptions::builder()
                .limit(i64::from(self.amount))
                .sort( doc! {"stats.views":-1})
                .build(),
            _ => FindOptions::builder()
                .limit(i64::from(self.amount))
                .sort( doc! {"time":-1})
                .build()
        };

        let paths: Vec<Document> = database.find(None, Some(find_options), self.amount).await;

        match paths.is_empty() {
            true => {
                self.show = None;
                self
            }
            false => {
                self.show = Some(Json(paths));
                self
            }
        }
    }
}

#[get("/view-posts/{page_number}/{sort}")]
pub async fn view_posts(
    path: Path<(u16, String)>,
    database: Data<mongodb::Collection<YuriPosts>>,
) -> Json<Vec<Document>> {
    let (page_number, sort) = path.into_inner();

    let mut generated = Gallery::new(page_number * 10);

    generated.gen_gallery(database, sort).await;

    generated.show.unwrap()
}


#[derive(Deserialize, Serialize, Debug)]
pub struct PostImageRequest {
    title: String,
    author: String,
    op: String,
    time: u64,
    tags: Option<Vec<String>>,
    file_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteImageRequest {
    title: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LikeImageRequest {
    path: String,
    title: String,
}

#[post("/post_image")]
pub async fn post_image(
    database: Data<mongodb::Collection<YuriPosts>>,
    request: Json<PostImageRequest>,
) -> HttpResponse {
    let path = format!(
        "./assets/posts/{}-{}-{}",
        &request.author, &request.time, &request.file_name
    );
    let database: mongodb::Collection<YuriPosts> = database.clone_with_type();

    let docs = YuriPosts {
        title: request.title.clone(),
        author: request.author.clone(),
        op: request.op.clone(),
        path,
        time: request.time.clone(),
        tags: request.tags.clone(),
        stats: PostStats::default(),
        comments: None,
    };

    database
        .insert_one(docs, None)
        .await
        .expect("Handle this error properly u lazy fuck");

    HttpResponse::Ok().body("yeet")
}

#[delete("/delete_post")]
pub async fn delete_post(
    database: Data<mongodb::Collection<YuriPosts>>,
    request: Json<DeleteImageRequest>,
) -> HttpResponse {
    let filter = doc! { "title": format!("{}", &request.title) };

    database
        .delete_one(filter, None)
        .await
        .expect("Handle this pweeze");

    HttpResponse::Ok().body("Deleted")
}

#[put("/like-post")]
pub async fn like_post(
        request: Json<LikeImageRequest>,
        database: Data<mongodb::Collection<YuriPosts>>
) -> HttpResponse {
    let filter = doc! {
        "title": format!("{}", &request.title),
        "path": format!("{}", &request.path)
    };
    let add_like = doc! { "$inc": { "stats.likes": 1 } };

    database
        .update_one(filter, add_like, None)
        .await
        .expect("Failed to add like");

    HttpResponse::Ok().body("HTTP/1.1 201 Updated")
}

#[put("/unlike-post")]
pub async fn unlike_post(
        request: Json<LikeImageRequest>,
        database: Data<mongodb::Collection<YuriPosts>>
) -> HttpResponse {
    let filter = doc! {
        "title": format!("{}", &request.title),
        "path": format!("{}", &request.path)
    };
    let remove_like = doc! { "$inc": { "stats.likes": -1 } };

    database
        .update_one(filter, remove_like, None)
        .await
        .expect("Failed to remove like");

    HttpResponse::Ok().body("HTTP/1.1 201 Updated")
}
