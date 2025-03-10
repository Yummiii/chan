use boards::BoardsRepository;
use posts::PostsRepository;
use sqlx::mysql::MySqlPoolOptions;

pub mod boards;
pub mod posts;

#[derive(Clone)]
pub struct Pools {
    pub boards: BoardsRepository,
    pub posts: PostsRepository,
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
        }
    }
}
