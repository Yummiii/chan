use crate::models::board::Board;
use sqlx::MySqlPool;

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

    pub async fn _get_by_id(&self, id: u64) -> Result<Board, sqlx::Error> {
        let board = sqlx::query_as::<_, Board>("SELECT * FROM boards WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(board)
    }

    pub async fn _create(&self, board: &Board) -> Result<Board, sqlx::Error> {
        let query = sqlx::query("INSERT INTO boards (slug, name) VALUES (?, ?)")
            .bind(&board.slug)
            .bind(&board.name);
        let res = query.execute(&self.pool).await?;

        self._get_by_id(res.last_insert_id()).await
    }
}
