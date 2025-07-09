// src/web/user_handler.rs

use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use std::sync::Arc; 

// Import your domain models (UserRequest, UserResponse)
use crate::domain::user::{UserRequest, UserResponse};


use crate::app::user_service::UserService; 

// --- HANDLER: Create User ---
pub async fn create_user(
    user_request: web::Json<UserRequest>,
    user_service: web::Data<Arc<dyn UserService>>, 
) -> impl Responder {
    match user_service.create_user(user_request.into_inner()).await {
        Ok(user_response) => {
            HttpResponse::Created().json(user_response)
        }
        Err(e) => {
            eprintln!("Error creating user: {:?}", e); 
            HttpResponse::InternalServerError().json(format!("Failed to create user: {}", e))
        }
    }
}

pub async fn get_all_users(
    user_service: web::Data<Arc<dyn UserService>>,
) -> impl Responder {
    match user_service.get_all_user().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Error retrieving all users: {:?}", e);
            HttpResponse::InternalServerError().json(format!("Failed to retrieve users: {}", e))
        }
    }
}
// --- HANDLER: Get User by ID ---> users/{id}---
pub async fn get_user_by_id(
    path: web::Path<Uuid>,
   
    user_service: web::Data<Arc<dyn UserService>>, 
) -> impl Responder {
    let user_id = path.into_inner();

    match user_service.get_user_by_id(user_id).await {
        Ok(Some(user_response)) => {
            HttpResponse::Ok().json(user_response)
        }
        Ok(None) => {
            HttpResponse::NotFound().json(format!("User with ID {} not found.", user_id))
        }
        Err(e) => {
            eprintln!("Error getting user by ID: {:?}", e);
            HttpResponse::InternalServerError().json(format!("Failed to retrieve user: {}", e))
        }
    }
}

// --- HANDLER: Get User by Email ---users?dd=gggg&df=234
pub async fn get_user_by_email(
    query: web::Query<std::collections::HashMap<String, String>>,
    
    user_service: web::Data<Arc<dyn UserService>>, 
) -> impl Responder {
    if let Some(email) = query.get("email") {
        match user_service.get_user_by_email(email).await {
            Ok(Some(user_response)) => {
                HttpResponse::Ok().json(user_response)
            }
            Ok(None) => {
                HttpResponse::NotFound().json(format!("User with email '{}' not found.", email))
            }
            Err(e) => {
                eprintln!("Error getting user by email: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Failed to retrieve user: {}", e))
            }
        }
    } else {
        HttpResponse::BadRequest().json("Email query parameter is missing.")
    }
}


// --- HANDLER: Update User ---
pub async fn update_user(
    path: web::Path<Uuid>,
    user_request: web::Json<UserRequest>,
    // This type MUST match what's provided by App::app_data()
    user_service: web::Data<Arc<dyn UserService>>, // <--- CORRECTED TYPE
) -> impl Responder {
    let user_id = path.into_inner();

    match user_service.update_user(user_id, user_request.into_inner()).await {
        Ok(user_response) => {
            HttpResponse::Ok().json(user_response)
        }
        Err(e) => {
            eprintln!("Error updating user: {:?}", e);
            HttpResponse::InternalServerError().json(format!("Failed to update user: {}", e))
        }
    }
}

// --- HANDLER: Delete User ---
pub async fn delete_user(
    path: web::Path<Uuid>,
    user_service: web::Data<Arc<dyn UserService>>, 
) -> impl Responder {
    let user_id = path.into_inner();

    match user_service.delete_user(user_id).await {
        Ok(true) => {
            HttpResponse::NoContent().finish()
        }
        Ok(false) => {
            HttpResponse::NotFound().json(format!("User with ID {} not found for deletion.", user_id))
        }
        Err(e) => {
            eprintln!("Error deleting user: {:?}", e);
            HttpResponse::InternalServerError().json(format!("Failed to delete user: {}", e))
        }
    }
}