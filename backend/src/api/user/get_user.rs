use common::mongodb::structs::Users;

use actix_web::{get, web::{Path, Json}, web::Data};
use bson::{oid::ObjectId, doc};

use crate::database::mongo::{MongodbDatabase, CollectionList};

#[get("/get_user/{user_id}")]
pub async fn get_user(path: Path<String>, database: Data<MongodbDatabase>) -> actix_web::Result<Json<Users>> {
    let users_collection = database.collection::<Users>(CollectionList::Users);

    let oid = ObjectId::parse_str(&path.as_str()).unwrap();
    match users_collection.find_one(doc!( "_id": oid ), None).await {
        Ok(v) => match v {
            Some(v) => Ok(Json(v)),
            None => todo!("Get request error")
        },
        Err(e) => todo!("{e}")
    }
}
