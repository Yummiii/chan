use crate::models::image::Image;

#[derive(Clone)]
pub struct ImagesRepository {
    pool: sqlx::MySqlPool,
}

impl ImagesRepository {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Image, sqlx::Error> {
        let image = sqlx::query_as::<_, Image>("select * from images where id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(image)
    }

    pub async fn create(&self, image: &Image) -> Result<(), sqlx::Error> {
        sqlx::query("insert into images (id, data, mime) values (?, ?, ?)")
            .bind(&image.id)
            .bind(&image.data)
            .bind(&image.mime)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
