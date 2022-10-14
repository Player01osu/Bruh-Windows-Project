use actix_web::{
    get,
    web::Data,
    web::{Json, Path},
};
use bson::doc;
use common::mongodb::structs::{UsersSerialize as UsersSerialize, ImageStatesDeserialize};

use crate::{
    api::user::{Users, UsersDb},
    database::mongo::{CollectionList, MongodbDatabase},
};

#[get("/get_user/{user_id}")]
pub async fn get_user(
    path: Path<String>,
    database: Data<MongodbDatabase>,
) -> actix_web::Result<Option<Json<UsersSerialize>>> {
    let users_collection = database.collection::<UsersDb>(CollectionList::Users);
    let public = Users::hash_priv(&path.into_inner());
    match users_collection.find_one(doc!("public": public), None).await {
        // User found
        Ok(v) => match v {
            Some(v) =>{
                let user = UsersSerialize {
                    public: v.public,
                    image_states: v.image_states.into_iter().map(|i| {
                        ImageStatesDeserialize {
                            id: i.id.to_string(),
                            uploader: i.uploader,
                            like_state: i.like_state,
                        }
                    }).collect()
                };
                Ok(Some(Json(user)))
            },
            None => Ok(None),
        },
        Err(e) => todo!("{e}"),
    }
}
