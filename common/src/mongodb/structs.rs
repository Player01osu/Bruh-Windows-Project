use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct YuriPosts {
    pub title: String,
    pub author: String,
    pub op: String,
    pub tags: Option<Vec<String>>,
    pub path: String,
    pub comments: Option<Vec<Comment>>,
    pub stats: PostStats,
    pub time: u64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub commenter: String,
    pub body: String,
    pub time: u64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostStats {
    pub likes: u64,
    pub views: u64,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub enum ImageExpandState {
    Unfocus,
    Focus,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Id {
    #[serde(rename = "$oid")]
    oid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageRequest {
    #[serde(rename = "_id")]
    _id: Id,
    pub title: String,
    pub author: String,
    pub op: String,
    pub time: usize,
    pub tags: Option<Vec<String>>,
    pub stats: PostStats,
    pub path: String,
    pub comments: Option<Vec<Comment>>,
}

impl Default for YuriPosts {
    fn default() -> Self {
        Self {
            title: "No_Title".to_string(),
            author: "No_Author".to_string(),
            op: "Poster".to_string(),
            tags: None,
            path: "EMPTY".to_string(),
            comments: None,
            stats: PostStats::default(),
            time: 0,
        }
    }
}

impl Default for PostStats {
    fn default() -> Self {
        Self { likes: 0, views: 0 }
    }
}

impl Default for Comment {
    fn default() -> Self {
        Self {
            commenter: "Poster".to_string(),
            body: String::new(),
            time: 0,
        }
    }
}

pub enum Sort {
    New,
    Top,
    Views,
}
