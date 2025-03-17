use super::ApiTags;
use crate::{
    database::Pools,
    hdbe,
    models::post::Post,
    response::{Response, bad_request, ok},
};
use poem::web::{Data, Path};
use poem_openapi::OpenApi;

pub struct ThreadsController;

#[OpenApi(prefix_path = "/threads", tag = "ApiTags::Threads")]
impl ThreadsController {
    /// Get thread by id
    #[oai(path = "/:id", method = "get", operation_id = "threads-get")]
    async fn get_thread(&self, id: Path<u64>, pools: Data<&Pools>) -> Response<Vec<Post>> {
        let posts = hdbe!(pools.posts.get_thread(*id).await, "Thread not found");
        if posts.is_empty() {
            return bad_request("Thread not found");
        }

        ok(posts)
    }
}
