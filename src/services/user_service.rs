use crate::repository::user_repository::UserRepository;
use crate::models::user::User;
use sqlx::MySqlPool;

/// Serviço de negócio para operações de usuários.
///
/// Encapsula regras de negócio e orquestra operações entre
/// repository e outras camadas.
pub struct UserService;

impl UserService {
    /// Cria um novo usuário no banco de dados.
    ///
    /// # Parâmetros
    /// - `pool`: Conexão com o banco MySQL
    /// - `user`: Dados do usuário (já deve ter senha criptografada e validações feitas)
    ///
    /// # Retorna
    /// - ID do novo usuário criado
    /// - Erro do banco de dados se falhar
    pub async fn create_user(pool: &MySqlPool, user: User) -> Result<i32, sqlx::Error> {
        UserRepository::create(pool, &user).await
    }

    /// Lista todos os usuários cadastrados.
    pub async fn list_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
        UserRepository::list(pool).await
    }

    /// Atualiza os dados de um usuário existente.
    ///
    /// # Parâmetros
    /// - `pool`: Conexão com o banco MySQL
    /// - `user`: Dados do usuário (deve ter ID definido)
    ///
    /// # Retorna
    /// - Ok(()) se atualizado com sucesso
    /// - Erro se o usuário não possuir ID ou falhar na operação
    pub async fn update_user(pool: &MySqlPool, user: User) -> Result<(), sqlx::Error> {
        UserRepository::update(pool, &user).await
    }

    /// Remove um usuário pelo ID.
    pub async fn delete_user(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        UserRepository::delete(pool, id).await
    }

    /// Promove um usuário alterando seu cargo (role_id).
    pub async fn promote_user(pool: &MySqlPool, id: i32, new_role_id: i16) -> Result<(), sqlx::Error> {
        UserRepository::promote_user(pool, id, new_role_id).await
    }
}