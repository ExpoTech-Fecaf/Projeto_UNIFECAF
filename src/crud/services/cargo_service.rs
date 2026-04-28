use crate::repository::CargoRepository;
use sqlx::PgPool;

pub struct CargoService;

impl CargoService {
    pub async fn listar_cargos(pool: &PgPool) -> Result<Vec<crate::models::Cargo>, sqlx::Error> {
        CargoRepository::listar(pool).await
    }

    pub async fn atualizar_cargo(pool: &PgPool, cargo: Cargo) -> Result<(), sqlx::Error> {
        CargoRepository::atualizar(pool, &cargo).await
    }

    pub async fn deletar_cargo(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
        CargoRepository::deletar(pool, id).await
    }
}