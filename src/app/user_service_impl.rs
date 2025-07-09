use async_trait::async_trait;
use uuid::Uuid;
use sqlx::Error as SqlxError;

use crate::app::user_service::UserService;

use crate::domain::user::{User, UserRequest, UserResponse};

use crate::infra::repository::user_repo::UserRepository;
use crate::infra::repository::user_repo_impl::PostgresUserRepository; // Concrete repository

use argon2::{self, Config};


fn hash_password2(password: &str) -> Result<String, argon2::Error> {
    let salt = b"somesalt";
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config)
}
    
/*
async fn hash_password(password: &str) -> Result<String, rust_argon2::Error> {
    let mut salt = [0u8; 16]; // Generate a 16-byte salt
    thread_rng().fill_bytes(&mut salt); // Fill with random bytes

    // Default configuration for Argon2id, reasonable for most uses
    // You can customize Config::default() or create your own:
    // let config = Config {
    //     variant: rust_argon2::Variant::Argon2id,
    //     version: rust_argon2::Version::V13,
    //     mem_power: 12, // 2^12 = 4096 KB (4MB)
    //     time_cost: 3,
    //     lanes: 4,
    //     output_len: 32, // Output hash length in bytes
    //     hash_len: 32, // Deprecated, use output_len
    // };
    let config = Config::default(); // Use the default configuration

    Argon2::default() // si non custom config  `Argon2::new(config)`
        .hash_encoded(password.as_bytes(), &salt, &config)
}
*/
#[derive(Clone)] 
pub struct UserServiceImpl<R: UserRepository> {
    user_repo: R,
}

impl UserServiceImpl<PostgresUserRepository> { 
    pub fn new(user_repo: PostgresUserRepository) -> Self {
        UserServiceImpl { user_repo }
    }
}

#[async_trait]
impl UserService for UserServiceImpl<PostgresUserRepository> {
    async fn create_user(&self, mut new_user_request: UserRequest) -> Result<UserResponse, SqlxError> {
       // new_user_request.password = hash_password(&new_user_request.password).await;

        new_user_request.password = match hash_password2(&new_user_request.password) {
        Ok(hash) => hash,
        Err(e) => return Err(SqlxError::Protocol(format!("Password hashing failed: {}", e).into())),
    };
    
        let user = self.user_repo.create_user(new_user_request).await?;

        Ok(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        })
    }

    async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<UserResponse>, SqlxError> {
        let user_option = self.user_repo.get_user_by_id(user_id).await?;
        Ok(user_option.map(|user| UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }))
    }

async fn get_all_user(&self) -> Result<Vec<UserResponse>, SqlxError> {
   let users= self.user_repo.get_all_user().await?;
    let mut us:Vec<UserResponse> = vec![];
    for u in users {
        us.push(UserResponse{
            id: u.id,
            username: u.username,
            email: u.email,
            created_at: u.created_at,
        });
    }

   Ok(us)
}

    async fn get_user_by_email(&self, email: &str) -> Result<Option<UserResponse>, SqlxError> {
        let user_option = self.user_repo.get_user_by_email(email).await?;
        Ok(user_option.map(|user| UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        }))
    }

    async fn update_user(&self, user_id: Uuid, mut updated_user_request: UserRequest) -> Result<UserResponse, SqlxError> {

         updated_user_request.password = match hash_password2(&updated_user_request.password) {
        Ok(hash) => hash,
        Err(e) => return Err(SqlxError::Protocol(format!("Password hashing failed: {}", e).into())),
    };
        let user = self.user_repo.update_user(user_id, updated_user_request).await?;

        Ok(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        })
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<bool, SqlxError> {
        self.user_repo.delete_user(user_id).await
    }
}