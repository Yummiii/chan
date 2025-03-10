use boards::BoardsController;
use images::ImagesController;
use poem_openapi::{OpenApiService, Tags};
use posts::PostsController;
use test::TestController;

mod boards;
mod images;
mod posts;
mod test;

#[derive(Tags)]
pub enum ApiTags {
    /// Operations related to posts.
    Posts,
    Test,
    /// Operations related to boards.
    Boards,
    /// Endpoint for getting images.
    Images,
}

pub fn get_service() -> OpenApiService<
    (
        PostsController,
        TestController,
        BoardsController,
        ImagesController,
    ),
    (),
> {
    OpenApiService::new(
        (
            PostsController,
            TestController,
            BoardsController,
            ImagesController,
        ),
        "Chan",
        "1.0",
    )
    .server("http://localhost:3000")
}
