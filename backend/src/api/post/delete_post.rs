use common::mongodb::structs::{CommentSection, YuriPosts};
use crate::api::post::structs::DeleteImageRequest;

use actix_web::{
    delete,
    web::Json,
    web::Data, HttpResponse,
};
use bson::oid::ObjectId;
use mongodb::bson::doc;


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

