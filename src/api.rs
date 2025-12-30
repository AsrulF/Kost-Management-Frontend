use serde::{Deserialize};
use gloo_net::http::Request;
use uuid::Uuid;

#[derive(PartialEq, Deserialize, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
}

pub async fn login(
    username: String,
    password: String,
) -> Result<AuthResponse, ()> 
{
    let response = Request::post("http://127.0.0.1:8080/login")
        .json(&serde_json::json!({
            "username": username,
            "password": password
        }))
        .unwrap()
        .send()
        .await
        .map_err(|_| ())?;

    if response.status() == 200 {
        response.json::<AuthResponse>().await.map_err(|_| ())
    } else {
        Err(())
    }
}

