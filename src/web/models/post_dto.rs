use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::web::models::user_dto::UserDto;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct PostDto {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    
    pub user_id: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user:UserDto,
}