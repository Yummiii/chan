use crate::models::user::User;

#[derive(Clone)]
pub struct UsersRepository {
    pool: sqlx::MySqlPool,
}

impl UsersRepository {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(&self, id: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn create_user(&self, user: &User) -> Result<User, sqlx::Error> {
        sqlx::query("INSERT INTO users (id, username, pass_hash) VALUES (?, ?, ?)")
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.pass_hash)
            .execute(&self.pool)
            .await?;
        self.get_by_id(&user.id).await
    }

    pub async fn get_by_username(&self, username: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }
}
