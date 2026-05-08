use crate::repository::user_repository::UserRepository;
use crate::models::user::User;
use sqlx::MySqlPool;

/// Serviço de negócio para operações de usuários.
pub struct UserService;

impl UserService {
    /// Cria um novo usuário e retorna o ID gerado.
    pub async fn create_user(pool: &MySqlPool, user: User) -> Result<i32, sqlx::Error> {
        UserRepository::create(pool, &user).await
    }

    /// Lista todos os usuários cadastrados.
    pub async fn list_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
        UserRepository::list(pool).await
    }

    /// Atualiza os dados de um usuário existente.
    pub async fn update_user(pool: &MySqlPool, user: User) -> Result<(), sqlx::Error> {
        UserRepository::update(pool, &user).await
    }

    /// Remove um usuário pelo ID.
    pub async fn delete_user(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        UserRepository::delete(pool, id).await
    }

    /// Promove um usuário alterando seu cargo.
    pub async fn promote_user(pool: &MySqlPool, id: i32, new_role_id: i16) -> Result<(), sqlx::Error> {
        UserRepository::promote_user(pool, id, new_role_id).await
    }
}