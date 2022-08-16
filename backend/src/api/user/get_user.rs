use actix_web::{
    get,
    web::Data,
    web::{Json, Path},
    ResponseError,
};
use bson::{doc, oid::ObjectId};

use crate::{
    api::user::{Users, UsersDb},
    database::mongo::{CollectionList, MongodbDatabase},
};

#[get("/get_user/{user_id}")]
pub async fn get_user(
    path: Path<String>,
    database: Data<MongodbDatabase>,
) -> actix_web::Result<Option<Json<UsersDb>>> {
    let users_collection = database.collection::<UsersDb>(CollectionList::Users);
    let public = Users::hash_priv(&path.into_inner());
    match users_collection.find_one(doc!("public": public), None).await {
        // User found
        Ok(v) => match v {
            Some(v) =>{
                Ok(Some(Json(v)))
            },
            None => Ok(None),
        },
        Err(e) => todo!("{e}"),
    }
}
