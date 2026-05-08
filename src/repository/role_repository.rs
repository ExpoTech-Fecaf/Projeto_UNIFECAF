use sqlx::MySqlPool;
use crate::models::role::Role;

/// Repositório de acesso a dados para cargos.
///
/// Opera sobre a tabela `cargo` no banco de dados.
pub struct RoleRepository;

impl RoleRepository {
    /// Lista todos os cargos disponíveis no sistema.
    pub async fn list(pool: &MySqlPool) -> Result<Vec<Role>, sqlx::Error> {
        let roles = sqlx::query("SELECT id, nome FROM cargo")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|row| {
                use sqlx::Row;
                let id: u64 = row.get("id");
                Role {
                    id: id as i32,
                    name: row.get("nome"),
                }
            })
            .collect();
        Ok(roles)
    }

    /// Atualiza o nome de um cargo existente.
    pub async fn update(pool: &MySqlPool, role: &Role) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE cargo SET nome = ? WHERE id = ?"
        )
        .bind(role.name.as_str())
        .bind(role.id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Remove um cargo pelo ID.
    pub async fn delete(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM cargo WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
