use super::ApiTags;
use crate::{
    database::{Pools, boards::Board},
    response::{Response, ok},
};
use poem::web::Data;
use poem_openapi::OpenApi;

pub struct BoardsController;

#[OpenApi(prefix_path = "/boards", tag = "ApiTags::Boards")]
impl BoardsController {
    /// Gets all available boards.
    #[oai(path = "/", method = "get", operation_id = "boards-get")]
    async fn index(&self, pools: Data<&Pools>) -> Response<Vec<Board>> {
        let boards = pools.boards.get_all().await.unwrap();

        ok(boards)
    }
}
