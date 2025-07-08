
use async_trait::async_trait;
use uuid::Uuid;
use chrono::Utc;
use sqlx::{Pool, Postgres, Error}; // Import sqlx::Error here

use crate::domain::user::{User, UserRequest};
use crate::infra::repository::user_repo::UserRepository; 

/// An implementation of `UserRepository` for `sqlx::Pool<Postgres>`.
#[derive(Clone)] 
pub struct PostgresUserRepository {
    pool: Pool<Postgres>,
}

impl PostgresUserRepository {
    /// Creates a new `PostgresUserRepository` instance.
    pub fn new(pool: Pool<Postgres>) -> Self {
        PostgresUserRepository { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, new_user: UserRequest) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, username, email, password_hash, created_at, updated_at
            "#,
            Uuid::new_v4(),
            new_user.username,
            new_user.email,
            new_user.password, // REMEMBER: Hash this!
            Utc::now(),
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update_user(&self, user_id: Uuid, updated_user: UserRequest) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET username = $1, email = $2, password_hash = $3, updated_at = $4
            WHERE id = $5
            RETURNING id, username, email, password_hash, created_at, updated_at
            "#,
            updated_user.username,
            updated_user.email,
            updated_user.password, // REMEMBER: Hash this!
            Utc::now(),
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<bool, Error> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }
}