use crate::models::post::Post;

#[derive(Clone)]
pub struct PostsRepository {
    pool: sqlx::MySqlPool,
}

impl PostsRepository {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(&self, id: u64) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as::<_, Post>("select * from posts where id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(post)
    }

    pub async fn create(&self, post: &Post) -> Result<Post, sqlx::Error> {
        let query = sqlx::query("insert into posts (board_id, thread_id, user_id, content, image_id, created_at) values (?, ?, ?, ?, ?, ?)")
            .bind(post.board_id)
            .bind(post.thread_id)
            .bind(post.user_id)
            .bind(&post.content)
            .bind(&post.image_id)
            .bind(post.created_at);
        let res = query.execute(&self.pool).await?;

        self.get_by_id(res.last_insert_id()).await
    }

    pub async fn get_root_by_board(&self, board: u64) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as::<_, Post>(
            "select * from posts where board_id = ? and thread_id is null",
        )
        .bind(board)
        .fetch_all(&self.pool)
        .await?;
        Ok(posts)
    }

    pub async fn get_thread(&self, root_id: u64) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as::<_, Post>(
            "select * from posts where thread_id = ? or id = ? order by created_at",
        )
        .bind(root_id)
        .bind(root_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(posts)
    }
}
