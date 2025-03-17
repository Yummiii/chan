use sqlx::MySqlPool;
use crate::models::board::Board;

#[derive(Clone)]
pub struct BoardsRepository {
    pool: MySqlPool,
}

impl BoardsRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Board>, sqlx::Error> {
        let boards = sqlx::query_as::<_, Board>("SELECT * FROM boards")
            .fetch_all(&self.pool)
            .await?;

        Ok(boards)
    }

    pub async fn get_by_slug(&self, slug: &str) -> Result<Board, sqlx::Error> {
        let board = sqlx::query_as::<_, Board>("SELECT * FROM boards WHERE slug = ?")
            .bind(slug)
            .fetch_one(&self.pool)
            .await?;

        Ok(board)
    }
}
