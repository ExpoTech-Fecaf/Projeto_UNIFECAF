use crate::models::user::{User, UserType};
use crate::services::auth_service;
use crate::services::user_service::UserService;
use crate::validators::user_validator::UserValidator;
use axum::extract::{State, Json};
use sqlx::MySqlPool;
use axum::extract::Path;
use crate::repository::user_repository::UserRepository;
use axum::Extension;
use axum::http::StatusCode;

/// Payload de requisição para login.
#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Resposta do endpoint de login.
#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user_type: Option<UserType>,
}

/// Payload de requisição para registro de usuário.
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

/// Resposta dos endpoints de registro e atualização de usuário.
#[derive(serde::Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<i32>,
}

/// Handler de login.
///
/// Busca usuários no banco, autentica por username/senha e retorna o tipo de usuário.
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
        Err(_) => Json(LoginResponse {
            success: false,
            message: "Failure".to_string(),
            user_type: None,
        }),
    }
}

/// Handler de registro de novo usuário.
///
/// Valida username único, role_id, CPF e data de nascimento antes de criar.
pub async fn register(
    State(pool): State<MySqlPool>,
    Json(payload): Json<RegisterRequest>,
) -> Json<RegisterResponse> {
    // Validação 1: Username único
    if let Err(e) = UserValidator::validate_username_unique(&pool, &payload.username).await {
        return Json(RegisterResponse {
            success: false,
            message: e.message,
            user_id: None,
        });
    }

    // Validação 2: Role ID válido
    if let Err(e) = UserValidator::validate_role_id(payload.role_id) {
        return Json(RegisterResponse {
            success: false,
            message: e.message,
            user_id: None,
        });
    }

    // Validação 3: CPF válido
    if let Err(e) = UserValidator::validate_cpf(&payload.cpf) {
        return Json(RegisterResponse {
            success: false,
            message: e.message,
            user_id: None,
        });
    }

    // Validação 4: Data no formato dd/mm/YYYY
    let birth_date = match UserValidator::validate_and_parse_date(&payload.birth_date) {
        Ok(date) => date,
        Err(e) => return Json(RegisterResponse {
            success: false,
            message: e.message,
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

/// Handler para listagem de todos os usuários.
pub async fn list_users(
    State(pool): State<MySqlPool>,
) -> Json<Vec<User>> {
    match UserService::list_users(&pool).await {
        Ok(users) => Json(users),
        Err(_) => Json(vec![]), // Retorna lista vazia em caso de erro
    }
}

/// Handler para atualização de usuário.
///
/// Valida role_id, CPF, username único e CPF único antes de atualizar.
pub async fn update_user(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(mut payload): Json<User>,
) -> Json<RegisterResponse> {
    payload.id = Some(id);

    // 1. Validação de Role ID
    if let Err(e) = UserValidator::validate_role_id(payload.role_id) {
        return Json(RegisterResponse { success: false, message: e.message, user_id: None });
    }

    // 2. Validação de CPF (Formato e Dígitos)
    if let Err(e) = UserValidator::validate_cpf(&payload.cpf) {
        return Json(RegisterResponse { success: false, message: e.message, user_id: None });
    }

    // 3. Validação de Username Único (Corrigido para coluna 'username')
    let existing_username = sqlx::query("SELECT id FROM users WHERE username = ? AND id != ?")
        .bind(&payload.username)
        .bind(id)
        .fetch_optional(&pool)
        .await;

    if !payload.password_hash.is_empty() {
        match auth_service::hash_password(&payload.password_hash) {
            Ok(hashed) => payload.password_hash = hashed,
            Err(_) => return Json(RegisterResponse {
                success: false,
                message: "Erro ao processar a criptografia da senha".to_string(),
                user_id: None,
            }),
        }
    }

    if let Ok(Some(_)) = existing_username {
        return Json(RegisterResponse {
            success: false,
            message: "Este nome de usuário já está sendo usado por outra pessoa".to_string(),
            user_id: None,
        });
    }

    // 4. Validação de CPF Único (Importante!)
    let existing_cpf = sqlx::query("SELECT id FROM users WHERE cpf = ? AND id != ?")
        .bind(&payload.cpf)
        .bind(id)
        .fetch_optional(&pool)
        .await;

    if let Ok(Some(_)) = existing_cpf {
        return Json(RegisterResponse {
            success: false,
            message: "Este CPF já está cadastrado para outro usuário".to_string(),
            user_id: None,
        });
    }

    // 5. Mapeamento do UserType
    payload.user_type = match payload.role_id {
        1 => UserType::Admin,
        2 => UserType::Funcionario,
        3 => UserType::Gerente,
        _ => UserType::Funcionario,
    };

    // 6. Executa a atualização
    match UserService::update_user(&pool, payload).await {
        Ok(_) => Json(RegisterResponse {
            success: true,
            message: "Usuário atualizado com sucesso".to_string(),
            user_id: Some(id),
        }),
        Err(e) => Json(RegisterResponse {
            success: false,
            message: format!("Erro ao atualizar no banco: {}", e),
            user_id: None,
        }),
    }
}

/// Handler para busca de usuário por ID.
pub async fn get_user(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    match UserRepository::get_by_id(&pool, id).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err((axum::http::StatusCode::NOT_FOUND, "Usuário não encontrado".into())),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// Handler para remoção de usuário por ID.
pub async fn delete_user(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<axum::http::StatusCode, (axum::http::StatusCode, String)> {
    match UserService::delete_user(&pool, id).await {
        Ok(_) => Ok(axum::http::StatusCode::NO_CONTENT),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}


/// Payload para promoção de usuário.
#[derive(serde::Deserialize)]
pub struct PromoteRequest {
    pub users_id: i32,
    pub new_role_id: i16,
}

/// Handler para promoção de usuário.
///
/// Requer permissão de Admin. Altera o cargo do usuário alvo.
pub async fn promote_user(
    State(pool): State<MySqlPool>,
    Extension(user): Extension<User>,
    Json(payload): Json<PromoteRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, String)>{

    // Chama check_permission para verificar exigindo nível de Admin
    if !auth_service::check_permission(&user, UserType::Admin) {
        return Err((
            StatusCode::FORBIDDEN,
            "Access denied: Somente administradores podem promover usuários".to_string(),
        ));
    }

    // Se aprovado, chama o serviço de promoção
    match UserService::promote_user(&pool, payload.users_id, payload.new_role_id).await {
        Ok(_) => Ok(Json(RegisterResponse {
            success: true,
            message: "Usuário promovido com sucesso".to_string(),
            user_id: Some(payload.users_id),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao promover usuário: {}", e),
        ))
    }
}
