use super::mongo::MongodbDatabase;
use common::mongodb::structs::{Comment, PostStats, Resolution, Source, YuriPosts, CommentSection};

use actix_multipart::Multipart;
use actix_web::{
    delete, get,
    http::StatusCode,
    post, put, web,
    web::Json,
    web::Path,
    web::{Data, Query},
    Error, HttpResponse, HttpResponseBuilder, Responder,
};
use bson::oid::ObjectId;
use futures_util::TryStreamExt as _;
use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
    results::InsertOneResult,
};
use serde::{Deserialize, Serialize};
use std::io::Write;
use uuid::Uuid;

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

    async fn gen_gallery(
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

#[derive(Serialize, Deserialize)]
pub struct ViewPostsPath {
    page_number: u16,
    sort: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    #[serde(default)]
    query: String,
}

#[get("/view-posts/{page_number}/{sort}")]
pub async fn view_posts(
    path: Path<ViewPostsPath>,
    query: Query<SearchQuery>,
    database: Data<mongodb::Collection<YuriPosts>>,
) -> Json<Vec<Document>> {
    let page_number = path.page_number;
    let sort = &path.sort;
    let query = &query.query;

    let mut generated = Gallery::new(page_number * 10);

    generated.gen_gallery(database, &sort, &query).await;

    match generated.show {
        Some(documents) => documents,
        None => Json(Vec::default()),
    }
}

#[post("/post_image")]
pub async fn post_image(
    posts_collection: Data<mongodb::Collection<YuriPosts>>,
    comments_collection: Data<mongodb::Collection<CommentSection>>,
    mut payload: Multipart,
) -> actix_web::Result<impl Responder> {
    let utc_now = chrono::Utc::now();
    let time = utc_now.timestamp() as u64;

    // FIXME: This is probably not good
    let mut title = String::default();
    let mut author = String::default();
    let mut op = String::default();
    let mut material = String::default();
    let mut link: Option<String> = None;
    let mut width: usize = 480;
    let mut height: usize = 640;
    let mut tags: Option<Vec<String>> = None;
    let mut filename = String::default();
    let mut path = String::default();

    while let Some(mut field) = payload.try_next().await? {
        // Match chunks of field name to title
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
                    let chunk_to_str = std::str::from_utf8(&chunk)?;
                    link = match !chunk_to_str.is_empty() {
                        true => Some(String::from(chunk_to_str)),
                        false => None,
                    };
                }
            }
            "tags" => {
                if let Some(chunk) = field.try_next().await? {
                    let chunk_to_str = std::str::from_utf8(&chunk)?;
                    tags = match !chunk_to_str.is_empty() {
                        true => Some(
                            chunk_to_str
                                .split_terminator(',')
                                .map(|s| String::from(s.trim()))
                                .collect::<Vec<String>>(),
                        ),
                        false => None,
                    };
                }
            }
            "filename" => {
                if let Some(chunk) = field.try_next().await? {
                    filename = std::str::from_utf8(&chunk)?.to_owned();
                    filename = sanitize_filename::sanitize(filename);
                }
            }
            "width" => {
                if let Some(chunk) = field.try_next().await? {
                    width = std::str::from_utf8(&chunk)?.parse().unwrap_or(480);
                }
            }
            "height" => {
                if let Some(chunk) = field.try_next().await? {
                    height = std::str::from_utf8(&chunk)?.parse().unwrap_or(640);
                }
            }
            "image" => {
                let mut filepath = format!("./assets/posts/{author}-{time}-{filename}");
                filepath.retain(|c| c != '?');
                path = filepath.clone();

                let mut f = web::block(|| std::fs::File::create(filepath)).await??;

                while let Some(chunk) = field.try_next().await? {
                    f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
                }
            }
            _ => (),
        }
    }

    let resolution = Resolution { width, height };

    let stats = PostStats { likes: 0, views: 0 };

    let source = Source { material, link };

    // Have to check string here for some reason.
    let op = match op.is_empty() {
        true => String::from("monika"),
        false => op,
    };

    let comment_oid = ObjectId::new();

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
        comments: comment_oid,
    };

    let post_oid = posts_collection
        .insert_one(docs, None)
        .await
        .expect("Handle this error properly u lazy fuck")
        .inserted_id
        .as_object_id()
        .expect("Could not convert to ObjectId");

    match create_comment_section(comments_collection, &comment_oid, &post_oid).await {
        Ok(_) => (),
        Err(e) => todo!("{e}"),
    }

    let oid = post_oid.to_hex();

    Ok(web::Json(DeleteImageRequest { oid }))
}

async fn create_comment_section(
    comments_collection: Data<mongodb::Collection<CommentSection>>,
    comment_oid: &ObjectId,
    post_oid: &ObjectId,
) -> Result<InsertOneResult, mongodb::error::Error> {
    let comment_section = CommentSection {
        oid: *comment_oid,
        post_oid: *post_oid,
        comments: Some([].to_vec()),
    };

    comments_collection.insert_one(comment_section, None).await
}

#[delete("/delete_post")]
pub async fn delete_post(
    posts_collection: Data<mongodb::Collection<YuriPosts>>,
    comments_collection: Data<mongodb::Collection<CommentSection>>,
    request: Json<DeleteImageRequest>,
) -> HttpResponse {
    let oid = ObjectId::parse_str(&request.oid.as_str()).unwrap();
    let filter = doc! {
        "_id": oid,
    };

    let post_struct = posts_collection
        .find_one(filter.clone(), None)
        .await
        .expect("BRO WHAT DA HECK")
        .expect("Unable to find post from ObjectId");

    std::fs::remove_file(post_struct.path).unwrap_or(eprintln!("Unable to remove file"));

    let post = posts_collection
        .find_one_and_delete(filter, None)
        .await
        .expect("Unable to remove from posts collection")
        .unwrap();

    let query = doc! {
        "_id": post.comments,
    };
    comments_collection.delete_one(query, None).await.unwrap();

    HttpResponse::Ok().body("Deleted")
}

#[put("/like-post")]
pub async fn like_post(
    request: Json<LikeImageRequest>,
    database: Data<mongodb::Collection<YuriPosts>>,
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
    database: Data<mongodb::Collection<YuriPosts>>,
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

#[derive(Serialize, Deserialize)]
pub struct ViewComments {
    post_id: String,
}

#[get("/view-posts/{post_id}")]
pub async fn view_post_comments(
    path: Path<ViewComments>,
    comments_collection: Data<mongodb::Collection<CommentSection>>,
) -> actix_web::Result<Json<CommentSection>> {
    let filter = doc! {
        "_id": ObjectId::parse_str(&path.post_id.as_str()).unwrap(),
    };

    let comments = match match comments_collection.find_one(filter, None).await {
        Ok(v) => v,
        Err(e) => todo!("{e}"),
    } {
        Some(v) => v,
        None => CommentSection::default(),
    };

    Ok(web::Json(comments))
}

#[post("/post-comment/{post_id}")]
pub async fn post_comment(
    path: Path<ViewComments>,
    request: Json<Comment>,
    comments_collection: Data<mongodb::Collection<CommentSection>>,
) -> actix_web::Result<Json<CommentSection>> {
    let query = doc! {
        "_id": ObjectId::parse_str(&path.post_id.as_str()).unwrap(),
    };

    //let comment = Comment {
    //    commenter: request.commenter,
    //    body: request.body,
    //    time: request.time,
    //};

    let update = doc! {
        "$push": { "comments":
            {
                "commenter": &request.commenter,
                "body": &request.body,
            }
        }
    };

    //comments_collection.update_one(query, update, None).await;
    let comment = comments_collection.find_one_and_update(query, update, None).await.unwrap().unwrap();

    Ok(web::Json(comment))
}
