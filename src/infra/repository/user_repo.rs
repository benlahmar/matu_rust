use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::user::{User, UserRequest}; // Adjust the path as needed

#[async_trait]
pub trait UserRepository {
   
    async fn create_user(&self, new_user: UserRequest) -> Result<User, sqlx::Error>;
    
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;
    
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
    
    async fn update_user(&self, user_id: Uuid, updated_user: UserRequest) -> Result<User, sqlx::Error>;
    
    async fn delete_user(&self, user_id: Uuid) -> Result<bool, sqlx::Error>;
}

