use super::ApiTags;
use crate::{
    database::Pools,
    hdbe,
    models::{board::Board, board_overview::BoardOverview},
    response::{Response, ok},
};
use poem::web::{Data, Path};
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

    /// Gets the overview of a board by its slug. The overview has all the threads in the board.
    #[oai(
        path = "/:board/overview",
        method = "get",
        operation_id = "board-overview"
    )]
    async fn overview(&self, board: Path<String>, pools: Data<&Pools>) -> Response<BoardOverview> {
        let board = hdbe!(pools.boards.get_by_slug(&board).await, "Board not found");
        let posts = hdbe!(pools.posts.get_root_by_board(board.id).await);

        ok(BoardOverview { threads: posts })
    }
}
