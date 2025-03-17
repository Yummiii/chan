use auth::AuthController;
use boards::BoardsController;
use images::ImagesController;
use poem_openapi::{OpenApiService, Tags};
use posts::PostsController;
use test::TestController;
use threads::ThreadsController;

mod auth;
mod boards;
mod images;
mod posts;
mod test;
mod threads;

const SERVER_URL: &str = if cfg!(debug_assertions) {
    "http://localhost:3000"
} else {
    "https://api.zuraaa.com/sharo"
};

#[derive(Tags)]
pub enum ApiTags {
    /// Operations related to posts.
    Posts,
    Test,
    /// Operations related to boards.
    Boards,
    /// Endpoint for getting images.
    Images,
    /// Operations related to threads.
    Threads,
    Auth,
}

pub fn get_service() -> OpenApiService<
    (
        PostsController,
        TestController,
        BoardsController,
        ImagesController,
        ThreadsController,
        AuthController,
    ),
    (),
> {
    OpenApiService::new(
        (
            PostsController,
            TestController,
            BoardsController,
            ImagesController,
            ThreadsController,
            AuthController,
        ),
        "Chan",
        "1.0",
    )
    .server(SERVER_URL)
}
