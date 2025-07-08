use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::user::{User, UserRequest, UserResponse};
use sqlx::Error as SqlxError; 
#[async_trait]
 pub trait UserService: Send + Sync {
    async fn create_user(&self, new_user_request: UserRequest) -> Result<UserResponse, SqlxError>;

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<UserResponse>, SqlxError>;

    async fn get_user_by_email(&self, email: &str) -> Result<Option<UserResponse>, SqlxError>;

    async fn update_user(&self, user_id: Uuid, updated_user_request: UserRequest) -> Result<UserResponse, SqlxError>;

    async fn delete_user(&self, user_id: Uuid) -> Result<bool, SqlxError>;
}