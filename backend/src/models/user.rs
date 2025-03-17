use poem_openapi::Object;
use sqlx::prelude::FromRow;

#[derive(Object, FromRow, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub pass_hash: String,
}
