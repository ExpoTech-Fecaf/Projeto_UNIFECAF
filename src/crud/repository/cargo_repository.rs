use sqlx::PgPool;
use crate::models::Cargo;

pub struct CargoRepository;

impl CargoRepository {
    pub async fn listar(pool: &PgPool) -> Result<Vec<Cargo>, sqlx::Error> {
        let cargos = sqlx::query_as!(Cargo, "SELECT id, nome FROM cargo")
            .fetch_all(pool)
            .await?;
        Ok(cargos)
    }

    pub async fn atualizar(pool: &PgPool, cargo: &Cargo) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE cargo SET nome = $1 WHERE id = $2",
            cargo.nome,
            cargo.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn deletar(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM cargo WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}