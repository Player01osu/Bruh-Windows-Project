use common::mongodb::structs::Users;

use actix_web::{put, web::Json, web::Data};
use bson::oid::ObjectId;

use crate::database::mongo::{MongodbDatabase, CollectionList};

#[put("/generate_user")]
pub async fn generate_user(database: Data<MongodbDatabase>) -> actix_web::Result<Json<Users>> {
    let users_collection = database.collection::<Users>(CollectionList::Users);
    let user_id = ObjectId::new();
    let user = Users::new(user_id);

    match users_collection.insert_one(&user, None).await {
        Ok(_) => Ok(Json(user)),
        Err(e) => todo!("Fail to insert into mongodb {e}")
    }

}
