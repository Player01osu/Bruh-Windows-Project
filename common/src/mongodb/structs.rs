use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct YuriPosts {
    pub title: String,
    pub author: String,
    pub op: String,
    pub tags: Vec<String>,
    pub path: String,
    pub comments: Vec<Comment>,
    pub post_stats: PostStats,
    pub time: u64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Comment {
    pub op: String,
    pub body: String,
    pub time: u64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct PostStats {
    pub likes: u64,
    pub views: u64,
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
    pub post_stats: PostStats,
    pub path: String,
    pub comments: Vec<Comment>,
}

impl Default for YuriPosts {
    fn default() -> Self {
        Self {
            title: "No_Title".to_string(),
            author: "No_Author".to_string(),
            op: "No_OP".to_string(),
            tags: [String::new()].to_vec(),
            path: "EMPTY".to_string(),
            comments: [Comment::default()].to_vec(),
            post_stats: PostStats::default(),
            time: 0,
        }
    }
}

impl Default for PostStats {
    fn default() -> Self {
        Self {
            likes: 0,
            views: 0,
        }
    }
}

impl Default for Comment {
    fn default() -> Self {
        Self {
            op: "None".to_string(),
            body: String::new(),
            time: 0
        }
    }
}

pub enum ImageMessage {
    ToggleExpando(usize),
    QueryImages(Vec<ImageRequest>),
}
