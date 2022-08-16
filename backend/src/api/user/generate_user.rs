use actix_web::{put, web::Json, web::Data};

use crate::{database::mongo::{MongodbDatabase, CollectionList}, api::user::{Users, UsersDb}};

#[put("/generate_user")]
pub async fn generate_user(database: Data<MongodbDatabase>) -> actix_web::Result<Json<Users>> {
    let users_collection = database.collection::<UsersDb>(CollectionList::Users);
    let user = Users::new();
    let doc = UsersDb::new(user.public.clone());

    match users_collection.insert_one(&doc, None).await {
        Ok(_) => Ok(Json(user)),
        Err(e) => todo!("Fail to insert into mongodb {e}")
    }
}
