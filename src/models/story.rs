use crate::models::comment::Comment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Story {
    pub id: usize,
    pub title: String,
    pub points: Option<i32>,
    pub user: Option<String>,
    pub time: usize,
    pub time_ago: String,
    #[serde(alias = "type")]
    pub story_type: String,
    pub url: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub comments: Option<Vec<Comment>>,
    pub comments_count: Option<usize>,
}
