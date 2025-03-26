use boards::BoardsRepository;
use images::ImagesRepository;
use posts::PostsRepository;
use sqlx::mysql::MySqlPoolOptions;
use users::UsersRepository;

pub mod boards;
pub mod images;
pub mod posts;
pub mod users;

#[derive(Clone)]
pub struct Pools {
    pub boards: BoardsRepository,
    pub posts: PostsRepository,
    pub images: ImagesRepository,
    pub users: UsersRepository,
}

impl Pools {
    pub async fn init(url: &str) -> anyhow::Result<Self, String> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .map_err(|_| "Failed to connect to database".to_string())?;

        sqlx::migrate!()
            .run(&pool)
            .await
            .map_err(|_| "Failed to run migrations".to_string())?;
        // .expect("Failed to run migrations");

        Ok(Self {
            boards: BoardsRepository::new(pool.clone()),
            posts: PostsRepository::new(pool.clone()),
            images: ImagesRepository::new(pool.clone()),
            users: UsersRepository::new(pool.clone()),
        })
    }
}
