use poem_openapi::Object;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Object)]
pub struct Board {
    pub id: u64,
    pub name: String,
    pub slug: String,
}
