use awc::Client;
use awc::error::JsonPayloadError;
use uuid::Uuid;
use crate::web::models::user_dto::UserDto;
use std::sync::Arc; //  Arc

#[derive(Clone)]
pub struct UsersClient {
    client: Client,
    base_url: String,
}

impl UsersClient  {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<UserDto, JsonPayloadError> {
        let url = format!("{}/users/{}", self.base_url, user_id);
        
        let mut response = self.client
            .get(&url)
            .send()
            .await
            // Use .expect() to unwrap the result of .send().await
            // If the network request fails, the program will panic here.
            .expect("Failed to send request to Users service"); 

        // Deserialize the response.
        // If the JSON parsing fails, the program will panic here.
        response.json::<UserDto>().await
    }
}