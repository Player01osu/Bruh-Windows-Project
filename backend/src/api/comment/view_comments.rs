use common::mongodb::structs::CommentSection;

use actix_web::{
    get, web,
    web::Json,
    web::Path,
    web::Data,
};
use bson::oid::ObjectId;
use mongodb::bson::doc;

use crate::api::comment::structs::ViewComments;

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

