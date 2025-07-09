use actix_web::{web, HttpResponse, Responder};

use std::sync::Arc; 
use crate::domain::post::{Post,PostRequest};
use crate::app::post_service::PostService;

pub async fn create_post(
    post_request: web::Json<PostRequest>,
    post_service: web::Data<Arc<dyn PostService>>, 
) -> impl Responder {
    match post_service.create_post(post_request.into_inner()).await {
        Ok(post) => {
            HttpResponse::Created().json(post)
        }
        Err(e) => {
            eprintln!("Error creating user: {:?}", e); 
            HttpResponse::InternalServerError().json(format!("Failed to create post: {}", e))
        }
    }
}

pub async fn get_all_posts(
    post_service: web::Data<Arc<dyn PostService>>,
) -> impl Responder {
    match post_service.list_posts().await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            eprintln!("Error retrieving all posts: {:?}", e);
            HttpResponse::InternalServerError().json(format!("Failed to retrieve users: {}", e))
        }
    }
}