use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub struct Database {
    pub pool: MySqlPool,
}

impl Database {
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

        Self { pool }
    }
}
