mod domain;
mod infra;
mod db; 
mod app;
mod web;

use actix_web::middleware::Logger;
use actix_web::{web as aw_web, App, HttpServer, Responder, HttpResponse}; // Alias web to aw_web to avoid conflict with `web` directory

use dotenvy::dotenv;
use uuid::Uuid;
use crate::app::post_service::PostService;
use crate::app::post_service_impl::PostServiceImpl;
use crate::domain::user;
use crate::infra::repository::post_repo_impl::PostRepoImpl;
use crate::infra::repository::user_repo::UserRepository;
use crate::infra::repository::user_repo_impl::PostgresUserRepository;

use crate::app::user_service::UserService; // The service trait
use crate::app::user_service_impl::UserServiceImpl;

use crate::web::handlers::user_handler::{create_user, get_user_by_id, get_user_by_email, update_user, delete_user,get_all_users};
use crate::web::handlers::post_handler::{create_post,get_all_posts};

use std::sync::Arc;

pub struct AppState {
    user_service: Arc<UserServiceImpl<PostgresUserRepository>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("actix_web=debug,{}=debug,info"));



    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = db::create_pool(&database_url).await?;
    let user_repo = PostgresUserRepository::new(pool.clone());
     let new_user_request = crate::domain::user::UserRequest {
        username: "habib".to_string(),
     email: "habib@example.com".to_string(),
        password: "secure_password".to_string(), // hasher ca mais ds service 
    };
    match user_repo.create_user(new_user_request).await {
         Ok(user) => println!("Created user: {:?}", user),
         Err(e) => eprintln!("Error creating user: {}", e),
     }
     
  /*
    let user_id=Uuid::parse_str("0e246570-a855-407d-920e-85b1b4334538").unwrap();
    
    match user_repo.get_user_by_id(user_id).await {
        Ok(Some(user))=>{
            println!("Found user by id: ID={}, Username={}, Email={}",
                     user.id, user.username, user.email);
        }
        Ok(None) => {
            println!("User avec id '{}' not found.", user_id);
        }
        Err(e) => {
            eprintln!(" Error finding user by id: {}", e);
        }
    }

    let email_to_find = "test@example.com"; 
    println!("\nAttempting to find user by email: {}", email_to_find);
    match user_repo.get_user_by_email(email_to_find).await {
        Ok(Some(user)) => {
            println!("Found user by email: ID={}, Username={}, Email={}",
                     user.id, user.username, user.email);
        }
        Ok(None) => {
            println!("User with email '{}' not found.", email_to_find);
        }
        Err(e) => {
            eprintln!(" Error finding user by email: {}", e);
        }
    }
     */
    // utilisation du service
   // let user_service = UserServiceImpl::new(user_repo);

    let concrete_service = UserServiceImpl::new(user_repo);
    let user_service: Arc<dyn UserService> = Arc::new(concrete_service);
    
    let myrepo=PostRepoImpl::new(pool.clone());
    let p_service = PostServiceImpl::new(myrepo);
    let post_service:Arc<dyn PostService> = Arc::new(p_service);
  /*
   match user_service.get_user_by_id(user_id).await {
         Ok(Some(user))=>{
            println!(" service user by id: ID={}, Username={}, Email={}",
                     user.id, user.username, user.email);
        }
        Ok(None) => {
            println!("User avec id '{}' not found.", user_id);
        }
        Err(e) => {
            eprintln!(" Error finding user by id: {}", e);
        }
    }
    */


    HttpServer::new(move || {
        App::new()
            // Register the shared service data
           // .app_data(user_service.clone()) // Clone for each worker thread
             .app_data(aw_web::Data::new(user_service.clone()))
              .app_data(aw_web::Data::new(post_service.clone()))
             .wrap(Logger::default())
            // User routes
            .service(
                aw_web::scope("/users") // Base path for user-related routes
                    .route("", aw_web::post().to(create_user)) // POST /users
                    .route("/{id}", aw_web::get().to(get_user_by_id)) // GET /users/{id}
                    .route("", aw_web::get().to(get_all_users)) // GET /allusers                    
                    .route("search", aw_web::get().to(get_user_by_email)) // GET /users?email=...
                    .route("/{id}", aw_web::put().to(update_user)) // PUT /users/{id}
                    .route("/{id}", aw_web::delete().to(delete_user)) // DELETE /users/{id}
            )
            .service(aw_web::scope("/posts")
                .route("", aw_web::post().to(create_post))
                .route("", aw_web::get().to(get_all_posts))
        
        )

            // Basic health check route
          //  .route("/health", aw_web::get().to(|| HttpResponse::Ok().body("OK")))
    })
    .bind(("127.0.0.1", 8083))?
    .run()
    .await?;


    Ok(()) 
}