use uuid::Uuid;
use async_trait::async_trait;
use crate::domain::post::{Post, PostRequest};

#[async_trait]
pub trait PostService : Send + Sync{
    async fn create_post(&self, post_req: PostRequest) -> Result<Post, sqlx::Error>;
    async fn get_post(&self, id: Uuid) -> Result<Option<Post>, sqlx::Error>;
    async fn update_post(&self, id: Uuid, post_req: PostRequest) -> Result<Option<Post>, sqlx::Error>;
    async fn delete_post(&self, id: Uuid) -> Result<(), sqlx::Error>;
    async fn list_posts(&self) -> Result<Vec<Post>, sqlx::Error>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Post>, sqlx::Error>;
    async fn search_by_keyword(&self, keyword: &str) -> Result<Vec<Post>, sqlx::Error>;
}