use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest{
   
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct UserResponse{
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>
}