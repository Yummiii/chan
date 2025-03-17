use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Object, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub pass_hash: String,
}
