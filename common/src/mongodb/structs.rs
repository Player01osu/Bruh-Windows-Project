use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// FIXME: Not all of these have to be pub (I think).
// Find all unnecessary uses of pub

// TODO: Reference counted to avoid large amounts of cloning?
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct YuriPosts {
    pub title: String,
    pub author: String,
    pub op: String,
    pub tags: Option<Vec<String>>,
    pub path: String,
    pub comments: ObjectId,
    pub stats: PostStats,
    pub source: Source,
    pub resolution: Resolution,
    pub time: u64,
}

#[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

#[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize)]
pub struct Source {
    pub material: String,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CommentSection {
    #[serde(rename = "_id")]
    pub oid: ObjectId,
    pub post_oid: ObjectId,
    pub comments: Option<Vec<Comment>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    #[serde(default = "default_user_id")]
    pub commenter: String,
    pub body: String,
}

fn default_user_id() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostStats {
    pub likes: u64,
    pub views: u64,
}

#[derive(Clone, PartialEq, Eq, Deserialize, Debug)]
pub enum ImageExpandState {
    Unfocus,
    Focus,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Id {
    #[serde(rename = "$oid")]
    pub oid: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageRequest {
    #[serde(rename = "_id")]
    pub _id: Id,
    pub title: String,
    pub author: String,
    pub op: String,
    pub time: usize,
    pub tags: Option<Vec<String>>,
    pub stats: PostStats,
    pub path: String,
    pub source: Source,
    pub resolution: Resolution,
    pub comments: Option<ObjectId>,
}

impl Default for YuriPosts {
    fn default() -> Self {
        Self {
            title: "No_Title".to_string(),
            author: "No_Author".to_string(),
            op: "Poster".to_string(),
            tags: None,
            path: "EMPTY".to_string(),
            comments: ObjectId::new(),
            source: Default::default(),
            resolution: Default::default(),
            stats: PostStats::default(),
            time: 0,
        }
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Self {
            width: 640,
            height: 480,
        }
    }
}

impl Default for Source {
    fn default() -> Self {
        Self {
            material: "None".to_string(),
            link: None,
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
        }
    }
}

pub enum Sort {
    New,
    Top,
    Views,
}
