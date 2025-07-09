
use async_trait::async_trait;
use uuid::Uuid;

use crate::app::post_service::PostService;
use crate::app::user_service::UserService;
use crate::domain::post::{Post, PostRequest};
use crate::infra::repository::post_repo::PostRepository;
use crate::infra::repository::post_repo_impl::PostRepoImpl;


 #[derive(Clone)] 
pub struct PostServiceImpl<P: PostRepository>
{
    post_repo:P,
    //user_service:UserService;
}
impl PostServiceImpl<PostRepoImpl>
{
    pub fn new(myrepo:PostRepoImpl)-> Self
    {
           PostServiceImpl {post_repo:myrepo} 
    }
}

#[async_trait]
impl PostService for PostServiceImpl<PostRepoImpl> {
     async fn create_post(&self, post_req: PostRequest) -> Result<Post, sqlx::Error>
     {
          let post=self.post_repo.create_post(post_req).await?;

          Ok(post)
     }


    async fn get_post(&self, id: Uuid) -> Result<Option<Post>, sqlx::Error>{
       let op= self.post_repo.get_post(id).await?;
       Ok(op)

    }
    async fn update_post(&self, id: Uuid, post_req: PostRequest) -> Result<Option<Post>, sqlx::Error>{
            let p=self.post_repo.update_post(id, post_req).await?;
            Ok(p)
    }
    async fn delete_post(&self, id: Uuid) -> Result<(), sqlx::Error>
    {
        self.post_repo.delete_post(id);
        Ok(())

    }
    async fn list_posts(&self) -> Result<Vec<Post>, sqlx::Error>{
          let ps=  self.post_repo.list_posts().await?;
          
          Ok(ps)
    }
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Post>, sqlx::Error>
    {
            //verification de user id;

            let ps=self.post_repo.find_by_user_id(user_id).await?;
            Ok(ps)

    }
    async fn search_by_keyword(&self, keyword: &str) -> Result<Vec<Post>, sqlx::Error>{
        let ps=self.post_repo.search_by_keyword(keyword).await?;
        Ok(ps)
    }
}