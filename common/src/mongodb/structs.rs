use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct YuriPosts {
    pub title: String,
    pub author: String,
    pub op: String,
    pub tags: Vec<String>,
    pub path: String,
    pub time: u64,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub enum ImageExpandState {
    Unfocus,
    Focus,
}


#[derive(Deserialize, Serialize, Debug)]
struct Id {
    #[serde(rename = "$oid")]
    oid: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ImageRequest {
    #[serde(rename = "_id")]
    _id: Id,
    pub title: String,
    pub author: String,
    pub op: String,
    pub time: usize,
    pub tags: Vec<String>,
    pub path: String,
}


pub enum ImageMessage {
    ToggleExpando(usize),
    QueryImages(Vec<ImageRequest>),
}
