use crate::repository::role_repository::RoleRepository;
use crate::models::role::Role;
use sqlx::MySqlPool;

pub struct RoleService;

impl RoleService {
    pub async fn list_roles(pool: &MySqlPool) -> Result<Vec<Role>, sqlx::Error> {
        RoleRepository::list(pool).await
    }

    pub async fn update_role(pool: &MySqlPool, role: Role) -> Result<(), sqlx::Error> {
        RoleRepository::update(pool, &role).await
    }

    pub async fn delete_role(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        RoleRepository::delete(pool, id).await
    }
}