use poem_openapi::Object;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Object)]
pub struct Post {
    pub id: u64,
    pub content: String,
    pub created_at: i64,
    pub board_id: u64,
    pub user_id: Option<String>,
    pub thread_id: Option<u64>,
    pub image_id: Option<String>,
}
