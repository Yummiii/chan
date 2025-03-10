use super::ApiTags;
use poem_openapi::{OpenApi, payload::PlainText};

pub struct TestController;

#[OpenApi(tag = "ApiTags::Test")]
impl TestController {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self) -> PlainText<String> {
        PlainText("hello".to_string())
    }
}
