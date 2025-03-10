use boards::BoardsController;
use poem_openapi::{OpenApiService, Tags};
use posts::PostsController;
use test::TestController;

mod boards;
mod posts;
mod test;

#[derive(Tags)]
pub enum ApiTags {
    /// Operations related to posts.
    Posts,
    Test,
    /// Operations related to boards.
    Boards,
}

pub fn get_service() -> OpenApiService<(PostsController, TestController, BoardsController), ()> {
    OpenApiService::new(
        (PostsController, TestController, BoardsController),
        "Chan",
        "1.0",
    )
    .server("http://localhost:3000")
}
