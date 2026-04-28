use crate::models::user::{User, UserType};
use crate::services::auth_service;
use crate::services::user_service::UserService;
use axum::extract::{State, Json};
use sqlx::MySqlPool;
use sqlx::Row;
use chrono::NaiveDate;

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

// Estrutura para representar uma requisição de registro
#[derive(serde::Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: String,
    pub cpf: String,
    pub role_id: i16,
}

// Estrutura para representar a resposta do registro
#[derive(serde::Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<i32>,
}

// Função para lidar com a lógica de login
pub async fn login(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> Json<LoginResponse> {
    // Load users from DB
    let users: Vec<User> = sqlx::query(
        "SELECT id, user as username, senha as password_hash, CASE fkidcargo WHEN 1 THEN 'Admin' WHEN 2 THEN 'Funcionario' WHEN 3 THEN 'Gerente' END as user_type, nome as first_name, sobrenome as last_name, datanascimento as birth_date, cpf, fkidcargo as role_id FROM usuario",
    )
    .fetch_all(&pool)
    .await
    .unwrap_or(vec![])
    .into_iter()
    .map(
        |row| {
            let user_id: u64 = row.get("id");
            User {
                id: Some(user_id as i32),
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
                cpf: row.get("cpf"),
                role_id: row.get("role_id"),
            }
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

// Função para lidar com a lógica de registro
pub async fn register(
    State(pool): State<MySqlPool>,
    Json(payload): Json<RegisterRequest>,
) -> Json<RegisterResponse> {
    let birth_date = match NaiveDate::parse_from_str(&payload.birth_date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return Json(RegisterResponse {
            success: false,
            message: "Data de nascimento inválida".to_string(),
            user_id: None,
        }),
    };

    let user_type = match payload.role_id {
        1 => UserType::Admin,
        2 => UserType::Funcionario,
        3 => UserType::Gerente,
        _ => UserType::Funcionario,
    };

    let password_hash = match auth_service::hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(_) => return Json(RegisterResponse {
            success: false,
            message: "Erro ao processar senha".to_string(),
            user_id: None,
        }),
    };

    let user = User {
        id: None,
        username: payload.username,
        password_hash,
        user_type,
        first_name: payload.first_name,
        last_name: payload.last_name,
        birth_date,
        cpf: payload.cpf,
        role_id: payload.role_id,
    };

    match UserService::create_user(&pool, user).await {
        Ok(id) => Json(RegisterResponse {
            success: true,
            message: "Usuário registrado com sucesso".to_string(),
            user_id: Some(id),
        }),
        Err(e) => Json(RegisterResponse {
            success: false,
            message: format!("Erro ao registrar usuário: {:?}", e),
            user_id: None,
        }),
    }
}
