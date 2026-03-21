use crate::models::user::{User, UserType};
use crate::services::auth_service;
use std::error::Error;

// Estrutura para representar uma requisição de login
#[derive(Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// Estrutura para representar a resposta do login
#[derive(Debug)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user_type: Option<UserType>,
}

// Função para lidar com a lógica de login
pub fn login(users: &[User], username: &str, password: &str) -> LoginResponse {
    match auth_service::authenticate_user(users, username, password) {
        Ok(user) => LoginResponse {
            success: true,
            message: "Success".to_string(),
            user_type: Some(user.user_type.clone()),
        },
        Err(error) => LoginResponse {
            success: false,
            message: "Failure".to_string(),
            user_type: None,
        },
    }
}
