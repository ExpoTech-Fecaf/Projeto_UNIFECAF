// ==========================================
// auth_handler.rs - Camada HTTP
// ==========================================
// Responsabilidade: Apenas receber requisições HTTP e retornar respostas
// NÃO valida dados, NÃO acessa banco direto

use crate::models::user::{User, UserType};
use crate::services::auth_service;
use crate::repository::user_repository::UserRepository;
use axum::extract::{State, Json, Path};
use axum::Extension;
use axum::http::StatusCode;
use sqlx::MySqlPool;

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user_type: Option<UserType>,
}

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

#[derive(serde::Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<i32>,
}

#[derive(serde::Deserialize)]
pub struct PromoteRequest {
    pub users_id: i32,
    pub new_role_id: i16,
}

/// Handler de login - apenas HTTP
pub async fn login(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> Json<LoginResponse> {
    match auth_service::login(&pool, &payload.username, &payload.password).await {
        Ok(user) => Json(LoginResponse {
            success: true,
            message: "Success".to_string(),
            user_type: Some(user.user_type.clone()),
        }),
        Err(e) => Json(LoginResponse {
            success: false,
            message: e,
            user_type: None,
        }),
    }
}

/// Handler de registro - delega para auth_service
pub async fn register(
    State(pool): State<MySqlPool>,
    Json(payload): Json<RegisterRequest>,
) -> Json<RegisterResponse> {
    match auth_service::register(
        &pool,
        &payload.username,
        &payload.password,
        &payload.first_name,
        &payload.last_name,
        &payload.birth_date,
        &payload.cpf,
        payload.role_id,
    ).await {
        Ok(id) => Json(RegisterResponse {
            success: true,
            message: "Usuário registrado com sucesso".to_string(),
            user_id: Some(id),
        }),
        Err(e) => Json(RegisterResponse {
            success: false,
            message: e,
            user_id: None,
        }),
    }
}

/// Handler para listar usuários
pub async fn list_users(
    State(pool): State<MySqlPool>,
) -> Json<Vec<User>> {
    match UserRepository::list(&pool).await {
        Ok(users) => Json(users),
        Err(_) => Json(vec![]),
    }
}

/// Handler para atualizar usuário
pub async fn update_user(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(mut payload): Json<User>,
) -> Json<RegisterResponse> {
    payload.id = Some(id);
    
    match auth_service::update_user(&pool, payload).await {
        Ok(_) => Json(RegisterResponse {
            success: true,
            message: "Usuário atualizado com sucesso".to_string(),
            user_id: Some(id),
        }),
        Err(e) => Json(RegisterResponse {
            success: false,
            message: e,
            user_id: None,
        }),
    }
}

/// Handler para obter usuário
pub async fn get_user(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    match UserRepository::get_by_id(&pool, id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((StatusCode::NOT_FOUND, "Usuário não encontrado".into())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Handler para deletar usuário
pub async fn delete_user(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    match UserRepository::delete(&pool, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Handler para promover usuário
pub async fn promote_user(
    State(pool): State<MySqlPool>,
    Extension(user): Extension<User>,
    Json(payload): Json<PromoteRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, String)> {
    if !auth_service::check_permission(&user, UserType::Admin) {
        return Err((
            StatusCode::FORBIDDEN,
            "Access denied: Somente administradores podem promover usuários".to_string(),
        ));
    }

    match UserRepository::promote_user(&pool, payload.users_id, payload.new_role_id).await {
        Ok(_) => Ok(Json(RegisterResponse {
            success: true,
            message: "Usuário promovido com sucesso".to_string(),
            user_id: Some(payload.users_id),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao promover usuário: {}", e),
        )),
    }
}


// ==========================================
// auth_service.rs - Lógica de Autenticação
// ==========================================
// Responsabilidade: Orquestrar login, register, update_user
// Chama validators, user_service, repository

use crate::models::user::{User, UserType};
use crate::repository::user_repository::UserRepository;
use crate::services::user_service::UserService;
use crate::validators::user_validator::UserValidator;
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::MySqlPool;

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, stored: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, stored)
}

pub fn check_permission(user: &User, required_type: UserType) -> bool {
    user.user_type.nivel() >= required_type.nivel()
}

/// Login: busca usuário por username e valida senha
pub async fn login(pool: &MySqlPool, username: &str, password: &str) -> Result<User, String> {
    let user = UserRepository::find_by_username(pool, username)
        .await
        .map_err(|_| "Erro ao buscar usuário")?
        .ok_or("Usuário não encontrado")?;

    if verify_password(password, &user.password_hash).unwrap_or(false) {
        Ok(user)
    } else {
        Err("Senha incorreta".to_string())
    }
}

/// Register: valida dados e cria novo usuário
pub async fn register(
    pool: &MySqlPool,
    username: &str,
    password: &str,
    first_name: &str,
    last_name: &str,
    birth_date: &str,
    cpf: &str,
    role_id: i16,
) -> Result<i32, String> {
    // Validações
    UserValidator::validate_username_unique(pool, username)
        .await
        .map_err(|e| e.message)?;
    
    UserValidator::validate_role_id(role_id).map_err(|e| e.message)?;
    UserValidator::validate_cpf(cpf).map_err(|e| e.message)?;
    
    let birth_date_parsed = UserValidator::validate_and_parse_date(birth_date)
        .map_err(|e| e.message)?;

    let user_type = match role_id {
        1 => UserType::Admin,
        2 => UserType::Funcionario,
        3 => UserType::Gerente,
        _ => UserType::Funcionario,
    };

    let password_hash = hash_password(password).map_err(|_| "Erro ao processar senha")?;

    let user = User {
        id: None,
        username: username.to_string(),
        password_hash,
        user_type,
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        birth_date: birth_date_parsed,
        cpf: cpf.to_string(),
        role_id,
    };

    UserService::create_user(pool, user)
        .await
        .map_err(|e| format!("Erro ao registrar usuário: {}", e))
}

/// Update: valida dados e atualiza usuário
pub async fn update_user(pool: &MySqlPool, mut user: User) -> Result<(), String> {
    UserValidator::validate_role_id(user.role_id).map_err(|e| e.message)?;
    UserValidator::validate_cpf(&user.cpf).map_err(|e| e.message)?;

    let user_id = user.id.ok_or("ID do usuário não definido")?;

    // Verifica username único
    let existing_username = sqlx::query("SELECT id FROM users WHERE username = ? AND id != ?")
        .bind(&user.username)
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| "Erro ao verificar username")?;

    if existing_username.is_some() {
        return Err("Este nome de usuário já está sendo usado por outra pessoa".to_string());
    }

    // Verifica CPF único
    let existing_cpf = sqlx::query("SELECT id FROM users WHERE cpf = ? AND id != ?")
        .bind(&user.cpf)
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| "Erro ao verificar CPF")?;

    if existing_cpf.is_some() {
        return Err("Este CPF já está cadastrado para outro usuário".to_string());
    }

    // Criptografa senha se fornecida
    if !user.password_hash.is_empty() {
        user.password_hash = hash_password(&user.password_hash)
            .map_err(|_| "Erro ao processar a criptografia da senha")?;
    }

    user.user_type = match user.role_id {
        1 => UserType::Admin,
        2 => UserType::Funcionario,
        3 => UserType::Gerente,
        _ => UserType::Funcionario,
    };

    UserService::update_user(pool, user)
        .await
        .map_err(|e| format!("Erro ao atualizar no banco: {}", e))
}


// ==========================================
// user_service.rs - Regras de Negócio
// ==========================================
// Responsabilidade: Orquestrar operações de usuário
// Chama repository

use crate::repository::user_repository::UserRepository;
use crate::models::user::User;
use sqlx::MySqlPool;

pub struct UserService;

impl UserService {
    /// Cria novo usuário
    pub async fn create_user(pool: &MySqlPool, user: User) -> Result<i32, sqlx::Error> {
        UserRepository::create(pool, &user).await
    }

    /// Lista todos os usuários
    pub async fn list_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
        UserRepository::list(pool).await
    }

    /// Atualiza usuário
    pub async fn update_user(pool: &MySqlPool, user: User) -> Result<(), sqlx::Error> {
        UserRepository::update(pool, &user).await
    }

    /// Deleta usuário
    pub async fn delete_user(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        UserRepository::delete(pool, id).await
    }

    /// Promove usuário
    pub async fn promote_user(pool: &MySqlPool, id: i32, new_role_id: i16) -> Result<(), sqlx::Error> {
        UserRepository::promote_user(pool, id, new_role_id).await
    }
}


// ==========================================
// user_repository.rs - Acesso ao Banco
// ==========================================
// Responsabilidade: Apenas SQLx queries
// Sem lógica de negócio

use sqlx::MySqlPool;
use sqlx::Row;
use crate::models::user::{User, UserType};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(pool: &MySqlPool, user: &User) -> Result<i32, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO users (username, password_hash, first_name, last_name, birth_date, cpf, role_id)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.birth_date)
        .bind(&user.cpf)
        .bind(&user.role_id)
        .execute(pool)
        .await?;
        
        let id = result.last_insert_id() as i32;
        Ok(id)
    }

    pub async fn list(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, first_name, last_name, cpf, birth_date, username, password_hash, role_id
            FROM users
            "#
        )
        .fetch_all(pool)
        .await?;

        let users = rows.into_iter().map(|row| {
            let role_id: i16 = row.get("role_id");
            User {
                id: Some(row.get("id")),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                birth_date: row.get("birth_date"),
                cpf: row.get("cpf"),
                username: row.get("username"),
                password_hash: row.get("password_hash"),
                user_type: match role_id {
                    1 => UserType::Admin,
                    2 => UserType::Funcionario,
                    3 => UserType::Gerente,
                    _ => UserType::Funcionario,
                },
                role_id,
            }
        }).collect();

        Ok(users)
    }

    pub async fn update(pool: &MySqlPool, user: &User) -> Result<(), sqlx::Error> {
        if let Some(user_id) = user.id {
            sqlx::query(
                r#"
                UPDATE users
                SET first_name = ?, last_name = ?, birth_date = ?,
                    username = ?, password_hash = ?, role_id = ?, cpf = ?
                WHERE id = ?
                "#,
            )
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(user.birth_date)
            .bind(&user.username)
            .bind(&user.password_hash)
            .bind(user.role_id)
            .bind(&user.cpf)
            .bind(user_id)
            .execute(pool)
            .await?;
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    pub async fn delete(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_by_id(pool: &MySqlPool, id: i32) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, username, first_name, last_name, cpf, birth_date, password_hash, role_id
            FROM users WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|row| {
            let role_id: i16 = row.get("role_id");
            User {
                id: Some(row.get("id")),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                birth_date: row.get("birth_date"),
                cpf: row.get("cpf"),
                username: row.get("username"),
                password_hash: row.get("password_hash"),
                user_type: match role_id {
                    1 => UserType::Admin,
                    2 => UserType::Funcionario,
                    3 => UserType::Gerente,
                    _ => UserType::Funcionario,
                },
                role_id,
            }
        }))
    }

    /// ✨ NOVO: Busca usuário por username
    pub async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, username, first_name, last_name, cpf, birth_date, password_hash, role_id
            FROM users WHERE username = ?
            "#
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|row| {
            let role_id: i16 = row.get("role_id");
            User {
                id: Some(row.get("id")),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                birth_date: row.get("birth_date"),
                cpf: row.get("cpf"),
                username: row.get("username"),
                password_hash: row.get("password_hash"),
                user_type: match role_id {
                    1 => UserType::Admin,
                    2 => UserType::Funcionario,
                    3 => UserType::Gerente,
                    _ => UserType::Funcionario,
                },
                role_id,
            }
        }))
    }

    pub async fn promote_user(pool: &MySqlPool, user_id: i32, new_role_id: i16) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET role_id = ? WHERE id = ?")
            .bind(new_role_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
