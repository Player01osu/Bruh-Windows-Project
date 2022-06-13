use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use actix::prelude::*;
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_files as fs;

use derive_more::Display;
use serde::{Deserialize, Serialize};

// TODO: Properly implement 404 Error.
pub async fn not_found() -> Result<NamedFile, actix_web::Error> {
    let path: PathBuf = "./files/not_found.html".parse().unwrap();
    //fs::Files::new("/", "./files/").index_file("index.html")
    Ok(NamedFile::open(path)?)
}


#[derive(Deserialize, Serialize)]
pub struct TaskIndentifier {
    task_global_id: String,
}

pub struct MyActor {
    pub count: usize,
}

#[get("/task/{task_global_id}")]
pub async fn get_task(task_indentifier: Path<TaskIndentifier>) -> Json<String> {
    return Json(task_indentifier.into_inner().task_global_id);
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Ping(pub usize);

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;

        self.count
    }
}
