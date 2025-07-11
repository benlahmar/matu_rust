use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use std::sync::Arc; 
use crate::domain::post::{self, Post, PostRequest};
use crate::app::post_service::PostService;
use crate::web::models::post_dto::PostDto;
use crate::web::users_client::UsersClient;

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


pub async fn get_post_withuser(
    path: web::Path<Uuid>,
    post_service: web::Data<Arc<dyn PostService>>,
    users_client: web::Data<UsersClient>, 
) -> impl Responder {
     let post_id = path.into_inner();

    //users_client.get_user(post_id);
    match post_service.get_post(post_id).await {
       Ok(Some(p)) => {
         let  mut post_dto =PostDto {  
            id: p.id,
                title: p.title,
                body: p.body,
                user_id: p.user_id,
                created_at: p.created_at,
                updated_at: p.updated_at
                , user: {crate::web::models::user_dto::UserDto { id: p.id, username: "".to_string(), email: "".to_string()}} };
           match  users_client.get_user(p.user_id).await{
            Ok(u)=>{
              post_dto.user=u;
                
               
             
            println!("post dto  {}, username: {}",post_dto.body, post_dto.user.username);
           HttpResponse::Ok().json("gggggg")
            }
            Err(e)=>{
                eprintln!("Error getting post by ID: {:?}", e);
            HttpResponse::InternalServerError().json(format!("Failed to retrieve user: {}", e))
     
            }
           };
            
            HttpResponse::Ok().json(post_dto)
        }
        Ok(None) => {
            HttpResponse::NotFound().json(format!("post with ID {} not found.",post_id))
        }
        Err(e) => {
            eprintln!("Error getting post by ID: {:?}", e);
            HttpResponse::InternalServerError().json(format!("Failed to retrieve post: {}", e))
        }
    }
}



pub async fn get_all_posts_user(
     path: web::Path<Uuid>,
    post_service: web::Data<Arc<dyn PostService>>,
) -> impl Responder {
     let id_user = path.into_inner();
    match post_service.find_by_user_id(id_user).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            eprintln!("Error retrieving all posts: {:?}", e);
            HttpResponse::InternalServerError().json(format!("Failed to retrieve users: {}", e))
        }
    }
}



pub async fn get_post_by_id(
    path: web::Path<Uuid>,
   
    post_service: web::Data<Arc<dyn PostService>>, 
) -> impl Responder {
    let id_post = path.into_inner();

    match post_service.get_post(id_post).await {
        Ok(Some(post_response)) => {
            HttpResponse::Ok().json(post_response)
        }
        Ok(None) => {
            HttpResponse::NotFound().json(format!("post with ID {} not found.", id_post))
        }
        Err(e) => {
            eprintln!("Error getting user by ID: {:?}", e);
            HttpResponse::InternalServerError().json(format!("Failed to retrieve user: {}", e))
        }
    }
}