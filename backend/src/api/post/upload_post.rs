use common::mongodb::structs::{CommentSection, PostStats, Resolution, Source, YuriPosts};
use crate::api::post::structs::DeleteImageRequest;

use actix_multipart::Multipart;
use actix_web::{
    post, web,
    web::Data, Responder,
};
use bson::oid::ObjectId;
use futures_util::TryStreamExt as _;
use mongodb::results::InsertOneResult;
use std::io::Write;

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

