use poem_openapi::Object;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Object)]
pub struct Post {
    pub id: u64,
    pub content: String,
    #[oai(rename = "createdAt")]
    pub created_at: i64,
    #[oai(rename = "boardId")]
    pub board_id: u64,
    #[oai(rename = "userId")]
    pub user_id: Option<String>,
    #[oai(rename = "threadId")]
    pub thread_id: Option<u64>,
    #[oai(rename = "imageId")]
    pub image_id: Option<String>,
}
