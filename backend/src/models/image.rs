use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Image {
    pub id: String,
    pub data: Vec<u8>,
    pub mime: String,
}
