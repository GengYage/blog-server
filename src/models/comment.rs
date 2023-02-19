use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Option<u64>,
    pub user_id: u64,
    pub article_id: u64,
    pub p_id: Option<u64>,
    pub content: String,
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentView {
    pub id: Option<u64>,
    pub user_id: u64,
    pub user_name: Option<String>,
    pub user_avatar_url: Option<String>,
    pub article_id: u64,
    pub p_id: Option<u64>,
    pub content: String,
    pub s_comment: Rc<RefCell<Vec<CommentView>>>,
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
}
