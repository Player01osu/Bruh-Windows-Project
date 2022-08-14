use common::mongodb::structs::YuriPosts;
use crate::api::post::structs::{ViewPostsPath, SearchQuery, Gallery};

use actix_web::{
    get,
    web::Json,
    web::Path,
    web::{Data, Query},
};
use mongodb::bson::Document;

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
