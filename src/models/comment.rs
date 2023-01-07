use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Comment {
    pub id: usize,
    pub level: usize,
    pub user: Option<String>,
    pub time: usize,
    pub time_ago: String,
    pub content: Option<String>,
    pub comments: Vec<Comment>,
}
