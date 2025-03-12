use crate::database::posts::Post;
use poem_openapi::Object;

#[derive(Debug, Object)]
pub struct BoardOverview {
    pub threads: Vec<Post>,
}
