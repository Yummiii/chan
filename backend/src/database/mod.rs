use boards::BoardsRepository;
use images::ImagesRepository;
use posts::PostsRepository;
use sqlx::mysql::MySqlPoolOptions;

pub mod boards;
pub mod images;
pub mod posts;

#[derive(Clone)]
pub struct Pools {
    pub boards: BoardsRepository,
    pub posts: PostsRepository,
    pub images: ImagesRepository,
}

impl Pools {
    pub async fn init(url: &str) -> Self {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        Self {
            boards: BoardsRepository::new(pool.clone()),
            posts: PostsRepository::new(pool.clone()),
            images: ImagesRepository::new(pool.clone()),
        }
    }
}
