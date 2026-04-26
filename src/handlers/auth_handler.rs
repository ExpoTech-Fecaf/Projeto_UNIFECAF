use crate::models::user::{User, UserType};
use crate::services::auth_service;
use axum::extract::{State, Json};
use sqlx::MySqlPool;
use sqlx::Row;

// Estrutura para representar uma requisição de login
#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// Estrutura para representar a resposta do login
#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user_type: Option<UserType>,
}

// Função para lidar com a lógica de login
pub async fn login(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> Json<LoginResponse> {
    // Load users from DB
    let users: Vec<User> = sqlx::query(
        "SELECT id, user as username, senha as password_hash, CASE fkidcargo WHEN 1 THEN 'Admin' WHEN 2 THEN 'Funcionario' WHEN 3 THEN 'Gerente' END as user_type, nome as first_name, sobrenome as last_name, datanascimento as birth_date FROM usuario",
    )
    .fetch_all(&pool)
    .await
    .unwrap_or(vec![])
    .into_iter()
    .map(
        |row| User {
            id: row.get("id"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            user_type: match row.get::<String, _>("user_type").as_str() {
                "Admin" => UserType::Admin,
                "Funcionario" => UserType::Funcionario,
                "Gerente" => UserType::Gerente,
                _ => UserType::Funcionario,
            },
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            birth_date: row.get("birth_date"),
        },
    )
    .collect();

    match auth_service::authenticate_user(&users, &payload.username, &payload.password) {
        Ok(user) => Json(LoginResponse {
            success: true,
            message: "Success".to_string(),
            user_type: Some(user.user_type.clone()),
        }),
        Err(_) => Json(LoginResponse {
            success: false,
            message: "Failure".to_string(),
            user_type: None,
        }),
    }
}
