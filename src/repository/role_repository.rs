use sqlx::MySqlPool;
use crate::models::role::Role;

pub struct RoleRepository;

impl RoleRepository {
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

    pub async fn delete(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM cargo WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
