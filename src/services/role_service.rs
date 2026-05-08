use crate::repository::role_repository::RoleRepository;
use crate::models::role::Role;
use sqlx::MySqlPool;

/// Serviço de negócio para operações de cargos.
pub struct RoleService;

impl RoleService {
    /// Lista todos os cargos disponíveis.
    pub async fn list_roles(pool: &MySqlPool) -> Result<Vec<Role>, sqlx::Error> {
        RoleRepository::list(pool).await
    }

    /// Atualiza o nome de um cargo.
    pub async fn update_role(pool: &MySqlPool, role: Role) -> Result<(), sqlx::Error> {
        RoleRepository::update(pool, &role).await
    }

    /// Remove um cargo pelo ID.
    pub async fn delete_role(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        RoleRepository::delete(pool, id).await
    }
}