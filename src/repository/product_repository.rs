use sqlx::MySqlPool;
use sqlx::Row;
use crate::models::product::Product;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn create(pool: &MySqlPool, product: &Product) -> Result<i32, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO produto (nome, valorcusto, valorvenda, estoqueatual, pesogramas, status, dataproducao, datavalidade)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(product.name.as_str())
        .bind(product.cost_price)
        .bind(product.sale_price)
        .bind(product.current_stock)
        .bind(product.weight_grams)
        .bind(product.status)
        .bind(product.production_date)
        .bind(product.expiration_date)
        .execute(pool)
        .await?;
        let row = sqlx::query("SELECT LAST_INSERT_ID() as id")
            .fetch_one(pool)
            .await?;
        let id: u64 = row.get("id");
        Ok(id as i32)
    }

    pub async fn list(pool: &MySqlPool) -> Result<Vec<Product>, sqlx::Error> {
        let products = sqlx::query(
            r#"
            SELECT id, nome, valorcusto, valorvenda, estoqueatual, pesogramas, status, dataproducao, datavalidade
            FROM produto
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| {
            let id: u64 = row.get("id");
            Product {
                id: id as i32,
                name: row.get("nome"),
                cost_price: row.get("valorcusto"),
                sale_price: row.get("valorvenda"),
                current_stock: row.get("estoqueatual"),
                weight_grams: row.get("pesogramas"),
                status: row.get("status"),
                production_date: row.get("dataproducao"),
                expiration_date: row.get("datavalidade"),
            }
        })
        .collect();
        Ok(products)
    }

    pub async fn update(pool: &MySqlPool, product: &Product) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE produto
            SET nome = ?, valorcusto = ?, valorvenda = ?, estoqueatual = ?,
                pesogramas = ?, status = ?, dataproducao = ?, datavalidade = ?
            WHERE id = ?
            "#,
        )
        .bind(product.name.as_str())
        .bind(product.cost_price)
        .bind(product.sale_price)
        .bind(product.current_stock)
        .bind(product.weight_grams)
        .bind(product.status)
        .bind(product.production_date)
        .bind(product.expiration_date)
        .bind(product.id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM produto WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
