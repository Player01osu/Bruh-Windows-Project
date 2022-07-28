use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web::Data, web::Json, web::Path};
use actix_web::{HttpResponse, HttpResponseBuilder};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use mongodb::bson::Document;
use mongodb::{bson::doc, options::FindOptions};

use common::mongodb::structs::{YuriPosts, PostStats, Comment, Resolution, Source};
use super::mongo::{MongodbDatabase};

#[derive(Deserialize, Serialize)]
pub struct TaskIndentifier {
    task_global_id: String,
}


pub struct Gallery {
    show: Option<Json<Vec<Document>>>,
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

    async fn gen_gallery(&mut self, database: Data<mongodb::Collection<YuriPosts>>, sort: String) -> &mut Self {

        let database = MongodbDatabase::new(database);

        let find_options = match sort.as_str() {
            "new" => FindOptions::builder()
                .limit(i64::from(self.amount))
                .sort( doc! {"time":-1})
                .build(),
            "top" => FindOptions::builder()
                .limit(i64::from(self.amount))
                .sort( doc! {"stats.likes":-1, "time":-1})
                .build(),
            "views" => FindOptions::builder()
                .limit(i64::from(self.amount))
                .sort( doc! {"stats.views":-1, "time":-1})
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

    // FIXME: Kinda need this to NOT panic when empty.
    generated.show.expect("Empty gallery")
}


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
pub struct DeleteImageRequest {
    title: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LikeImageRequest {
    oid: String,
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

    // TODO: Reference counted?
    let docs = YuriPosts {
        title: request.title.clone(),
        author: request.author.clone(),
        op: request.op.clone(),
        path,
        source: request.source.clone(),
        resolution: request.resolution.clone(),
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
    // Parse oid into ObjectId object type
    let oid = ObjectId::parse_str(&request.oid.as_str()).unwrap();
    let filter = doc! {
        "_id": oid,
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
    // Parse oid into ObjectId object type
    let oid = ObjectId::parse_str(&request.oid.as_str()).unwrap();
    let filter = doc! {
        "_id": oid,
    };
    let remove_like = doc! { "$inc": { "stats.likes": -1 } };

    database
        .update_one(filter, remove_like, None)
        .await
        .expect("Failed to remove like");

    HttpResponse::Ok().body("HTTP/1.1 201 Updated")
}
