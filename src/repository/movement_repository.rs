use sqlx::MySqlPool;
use sqlx::Row;
use crate::models::movement::Movement;

/// Repositório de acesso a dados para movimentações de estoque.
///
/// Registra e consulta o histórico de entradas e saídas de produtos.
pub struct MovementRepository;

impl MovementRepository {
    /// Registra uma nova movimentação (entrada ou saída) no histórico.
    pub async fn create(pool: &MySqlPool, movement: &Movement) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO movements (product_id, batch_id, user_id, movement_type, quantity, notes)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(movement.product_id)
        .bind(movement.batch_id)
        .bind(movement.user_id)
        .bind(&movement.movement_type)
        .bind(movement.quantity)
        .bind(&movement.notes)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Lista todas as movimentações ordenadas por data (mais recentes primeiro).
    pub async fn list_all(pool: &MySqlPool) -> Result<Vec<Movement>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, product_id, batch_id, user_id, movement_type, quantity, created_at, notes
            FROM movements
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        let movements = rows.into_iter().map(|row| {
            Movement {
                id: Some(row.get("id")),
                product_id: row.get("product_id"),
                batch_id: row.get("batch_id"),
                user_id: row.get("user_id"),
                movement_type: row.get("movement_type"),
                quantity: row.get("quantity"),
                created_at: row.get("created_at"),
                notes: row.get("notes"),
            }
        }).collect();

        Ok(movements)
    }

    /// Lista movimentações de um produto específico, ordenadas por data decrescente.
    pub async fn list_by_product(pool: &MySqlPool, product_id: i32) -> Result<Vec<Movement>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, product_id, batch_id, user_id, movement_type, quantity, created_at, notes
            FROM movements
            WHERE product_id = ?
            ORDER BY created_at DESC
            "#
        )
        .bind(product_id)
        .fetch_all(pool)
        .await?;

        let movements = rows.into_iter().map(|row| {
            Movement {
                id: Some(row.get("id")),
                product_id: row.get("product_id"),
                batch_id: row.get("batch_id"),
                user_id: row.get("user_id"),
                movement_type: row.get("movement_type"),
                quantity: row.get("quantity"),
                created_at: row.get("created_at"),
                notes: row.get("notes"),
            }
        }).collect();

        Ok(movements)
    }
}
