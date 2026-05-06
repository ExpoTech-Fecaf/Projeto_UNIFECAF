use sqlx::MySqlPool;
use sqlx::Row;
use crate::models::product::Product;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn create(pool: &MySqlPool, product: &Product) -> Result<i32, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO products (name, cost_price, sale_price, current_stock, weight_grams, status, production_date, expiration_date, entry_date)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
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
        .bind(product.entry_date)
        .execute(pool)
        .await?;

        Ok(result.last_insert_id() as i32)
    }

    pub async fn list(pool: &MySqlPool) -> Result<Vec<Product>, sqlx::Error> {
        let products = sqlx::query(
            r#"
            SELECT id, name, cost_price, sale_price, current_stock, weight_grams, status, production_date, expiration_date, entry_date
            FROM products
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| {
            Product {
                id: row.get("id"),
                name: row.get("name"),
                cost_price: row.get("cost_price"),
                sale_price: row.get("sale_price"),
                current_stock: row.get("current_stock"),
                weight_grams: row.get("weight_grams"),
                status: row.get("status"),
                production_date: row.get("production_date"),
                expiration_date: row.get("expiration_date"),
                entry_date: row.get("entry_date"),
            }
        })
        .collect();
        Ok(products)
    }

    pub async fn update(pool: &MySqlPool, product: &Product) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE products
            SET name = ?, cost_price = ?, sale_price = ?, current_stock = ?,
                weight_grams = ?, status = ?, production_date = ?, expiration_date = ?, entry_date = ?
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
        .bind(product.entry_date)
        .bind(product.id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Busca todos os lotes (produtos) de um tipo específico, ordenados por entry_date (ASC) para FIFO
    pub async fn find_batches_by_name(pool: &MySqlPool, product_name: &str) -> Result<Vec<Product>, sqlx::Error> {
        let batches = sqlx::query(
            r#"
            SELECT id, name, cost_price, sale_price, current_stock, weight_grams, status, production_date, expiration_date, entry_date
            FROM products
            WHERE name = ?
            ORDER BY entry_date ASC
            "#
        )
        .bind(product_name)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| {
            Product {
                id: row.get("id"),
                name: row.get("name"),
                cost_price: row.get("cost_price"),
                sale_price: row.get("sale_price"),
                current_stock: row.get("current_stock"),
                weight_grams: row.get("weight_grams"),
                status: row.get("status"),
                production_date: row.get("production_date"),
                expiration_date: row.get("expiration_date"),
                entry_date: row.get("entry_date"),
            }
        })
        .collect();

        Ok(batches)
    }

    /// Atualiza apenas a quantidade de estoque de um lote específico
    pub async fn update_quantity(pool: &MySqlPool, product_id: i32, new_quantity: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE products SET current_stock = ? WHERE id = ?")
            .bind(new_quantity)
            .bind(product_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
