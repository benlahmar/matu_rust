use chrono::Utc;
use uuid::Uuid;
use async_trait::async_trait;
use sqlx::{Pool, Postgres, Error}; 
use crate::domain::post::{Post, PostRequest};
use crate::infra::repository::post_repo::PostRepository;

pub struct PostRepoImpl {
    pub pool: Pool<Postgres>,
}
impl PostRepoImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        PostRepoImpl { pool }
    }
}

#[async_trait]
impl PostRepository for PostRepoImpl {
    async fn create_post(&self, post_req: PostRequest) -> Result<Post, Error> {
        let rec = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (id, user_id, title, body, created_at, updated_at)
            VALUES ($1, $2, $3, $4,$5,$6)
            RETURNING id, user_id, title, body,created_at, updated_at
            "#,
            Uuid::new_v4(),
            post_req.user_id,
            post_req.title,
            post_req.body,
             Utc::now(),
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(rec)
    }

    async fn get_post(&self, id: Uuid) -> Result<Option<Post>, Error> {
        let rec = sqlx::query_as!(
            Post,
            r#"SELECT id, user_id, title, body, created_at, updated_at FROM posts WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(rec)
    }

    async fn update_post(&self, id: Uuid, post_req: PostRequest) -> Result<Option<Post>, Error> {
        let rec = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET title = $1, body = $2,updated_at = $3
            WHERE id = $4
            RETURNING id, user_id, title, body, created_at, updated_at
            "#,
            post_req.title,
            post_req.body,
            Utc::now(),
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(rec)
    }

    async fn delete_post(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            r#"DELETE FROM posts WHERE id = $1"#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn list_posts(&self) -> Result<Vec<Post>, Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"SELECT id, user_id, title, body, created_at, updated_at FROM posts"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(posts)
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Post>, Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"SELECT id, user_id, title, body, created_at, updated_at FROM posts WHERE user_id = $1"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(posts)
    }

    async fn search_by_keyword(&self, keyword: &str) -> Result<Vec<Post>, Error> {
        let pattern = format!("%{}%", keyword);
        let posts = sqlx::query_as!(
            Post,
            r#"SELECT id, user_id, title, body, created_at, updated_at FROM posts WHERE title ILIKE $1 OR body ILIKE $1"#,
            pattern
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(posts)
    }
}