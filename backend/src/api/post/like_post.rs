use crate::{
    api::user::Users,
    database::mongo::{CollectionList, MongodbDatabase},
};
use common::mongodb::structs::{UsersDb, YuriPosts};
use serde::Deserialize;

use actix_web::{
    put,
    web::{Data, Path},
    HttpResponse,
};
use bson::oid::ObjectId;
use mongodb::bson::doc;

#[derive(Deserialize)]
pub struct PathInfo {
    oid: String,
    private: String,
}

#[put("/like-post/{oid}/{private}")]
pub async fn like_post(path: Path<PathInfo>, database: Data<MongodbDatabase>) -> HttpResponse {
    let users_collection = database.collection::<UsersDb>(CollectionList::Users);
    let posts_collection = database.collection::<YuriPosts>(CollectionList::Posts);

    // Parse oid into ObjectId object type
    let oid = ObjectId::parse_str(&path.oid.as_str()).unwrap();
    let filter = doc!("_id": oid);
    let add_like = doc! { "$inc": { "stats.likes": 1 } };

    let public = Users::hash_priv(&path.private);

    let update_query = doc!("public": &public, "image_states._id": &oid);
    let update_user = doc! (
        "$set": {
            "image_states.$.like_state": "Liked"
        }
    );

    match users_collection
        .update_one(update_query, update_user, None)
        .await
    {
        Ok(r) => {
            if r.modified_count == 0 {
                // Push if post_id doesn't exist
                let update_query = doc!(
                    "public": &public,
                );
                let update_user = doc! (
                    "$push": { "image_states": {
                        "_id": &oid,
                        "like_state": "Liked"
                        }
                    }
                );
                users_collection
                    .update_one(update_query, update_user, None)
                    .await
                    .expect("Fail to write to database");
            }
        }
        Err(e) => todo!("{e}"),
    };

    posts_collection
        .update_one(filter, add_like, None)
        .await
        .expect("Failed to add like");

    HttpResponse::Ok().body("HTTP/1.1 201 Updated")
}

#[put("/unlike-post/{oid}/{private}")]
pub async fn unlike_post(path: Path<PathInfo>, database: Data<MongodbDatabase>) -> HttpResponse {
    let posts_collection = database.collection::<YuriPosts>(CollectionList::Posts);
    let users_collection = database.collection::<UsersDb>(CollectionList::Users);

    // Parse oid into ObjectId object type
    let oid = ObjectId::parse_str(&path.oid.as_str()).unwrap();
    let filter = doc!("_id": oid);
    let remove_like = doc! { "$inc": { "stats.likes": -1 } };

    let public = Users::hash_priv(&path.private);

    let update_query = doc!("public": &public, "image_states._id": &oid);
    let update_user = doc! (
        "$set": {
            "image_states.$.like_state": "Unliked"
        }
    );

    match users_collection
        .update_one(update_query, update_user, None)
        .await
    {
        Ok(r) => {
            if r.modified_count == 0 {
                // Push if post_id doesn't exist
                let update_query = doc!(
                    "public": &public,
                );
                let update_user = doc! (
                    "$push": { "image_states": {
                        "_id": &oid,
                        "like_state": "Unliked"
                        }
                    }
                );
                users_collection
                    .update_one(update_query, update_user, None)
                    .await
                    .expect("Fail to write to database");
            }
        }
        Err(e) => todo!("{e}"),
    };

    posts_collection
        .update_one(filter, remove_like, None)
        .await
        .expect("Failed to remove like");

    HttpResponse::Ok().body("HTTP/1.1 201 Updated")
}
