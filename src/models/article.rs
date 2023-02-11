use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 文章信息
pub struct Article {
    pub id: Option<u64>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
}
