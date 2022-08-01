use std::io::Write;
use actix_multipart::Multipart;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::fs;
use futures_util::TryStreamExt as _;
use uuid::Uuid;
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web::Data, web::Json, web::Path};
use actix_web::{HttpResponse, HttpResponseBuilder, web};
use bson::oid::ObjectId;
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
    oid: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LikeImageRequest {
    oid: String,
}



#[post("/post_image")]
pub async fn post_image(
    database: Data<mongodb::Collection<YuriPosts>>,
    mut payload: Multipart,
) -> actix_web::Result<HttpResponse> {
    let database: mongodb::Collection<YuriPosts> = database.clone_with_type();

    let mut title = String::new();
    let mut author = String::new();
    let mut op = String::new();
    let mut material = String::new();
    let mut link = String::new();
    let mut width = String::new();
    let mut height = String::new();
    let mut tags = String::new();
    let mut time = String::new();
    let mut path = String::new();

    while let Some(mut field) = payload.try_next().await? {
        match field.name() {
            "title" => {
                if let Some(chunk) = field.try_next().await? {
                    title = std::str::from_utf8(&chunk)?.to_owned();
                }
            }
            "author" => {
                if let Some(chunk) = field.try_next().await? {
                    author = std::str::from_utf8(&chunk)?.to_owned();
                }
            }
            "op" => {
                if let Some(chunk) = field.try_next().await? {
                    op = std::str::from_utf8(&chunk)?.to_owned();
                }
            }
            "material" => {
                if let Some(chunk) = field.try_next().await? {
                    material = std::str::from_utf8(&chunk)?.to_owned();
                }
            }
            "link" => {
                if let Some(chunk) = field.try_next().await? {
                    link = std::str::from_utf8(&chunk)?.to_owned();
                }
            }
            "width" => {
                if let Some(chunk) = field.try_next().await? {
                    width = std::str::from_utf8(&chunk)?.to_owned();
                }
            }
            "height" => {
                if let Some(chunk) = field.try_next().await? {
                    height = std::str::from_utf8(&chunk)?.to_owned();
                }
            }
            "tags" => {
                if let Some(chunk) = field.try_next().await? {
                    tags = std::str::from_utf8(&chunk)?.to_owned();
                }
            }
            "time" => {
                if let Some(chunk) = field.try_next().await? {
                    time = std::str::from_utf8(&chunk)?.to_owned();
                }
            }
            "image" => {
                // A multipart/form-data stream has to contain `content_disposition`
                let content_disposition = field.content_disposition();

                let filename = content_disposition
                    .get_filename()
                    .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
                let filepath = format!("./assets/posts/{author}-{time}-{filename}");
                path = filepath.clone();

                let mut f = web::block(|| std::fs::File::create(filepath)).await??;

                while let Some(chunk) = field.try_next().await? {
                    f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
                }
            }
            _ => (),
        }
    }

    let resolution = Resolution {
        width: width.parse().unwrap(),
        height: height.parse().unwrap(),
    };

    let time = time.parse().unwrap();

    let link = match link.is_empty() {
        false => None,
        true => Some(link),
    };

    let tags = match tags.is_empty() {
        true => None,
        false => Some(tags.split_terminator(',')
            .map(|s| String::from(s))
            .collect::<Vec<String>>()),
    };

    let stats = PostStats {
        likes: 0,
        views: 0,
    };

    let source = Source {
        material,
        link,
    };
    //
    // TODO: Reference counted?
    let docs = YuriPosts {
        title,
        author,
        op,
        path,
        source,
        resolution,
        time,
        tags,
        stats,
        comments: None,
    };

    database
        .insert_one(docs, None)
        .await
        .expect("Handle this error properly u lazy fuck");

    Ok(HttpResponse::Ok().body("yeet"))
}

#[delete("/delete_post")]
pub async fn delete_post(
    database: Data<mongodb::Collection<YuriPosts>>,
    request: Json<DeleteImageRequest>,
) -> HttpResponse {
    let oid = ObjectId::parse_str(&request.oid.as_str()).unwrap();
    let filter = doc! {
        "_id": oid,
    };

    let post_struct = database.find_one(filter.clone(), None)
        .await
        .expect("BRO WHAT DA HECK")
        .expect("bruh how u get here");

    std::fs::remove_file(post_struct.path).unwrap();

    database.delete_one(filter, None)
        .await
        .expect("SHITTTT");

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
