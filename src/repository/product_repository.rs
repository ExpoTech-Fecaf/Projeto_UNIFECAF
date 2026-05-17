use sqlx::MySqlPool;
use sqlx::Row;
use serde::{Serialize, Deserialize};
use crate::models::product::Product;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchReport {
    pub id: i32,
    pub quantity: i32,
    pub entry_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockReport {
    pub product_id: i32,
    pub product_name: String,
    pub status: i16,
    pub total_stock: i32,
    pub batches: Vec<BatchReport>,
}

pub struct ProductRepository;

impl ProductRepository {
    fn row_to_product(row: sqlx::mysql::MySqlRow) -> Product {
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
    }

    pub async fn create(pool: &MySqlPool, product: &Product) -> Result<i32, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO products (name, cost_price, sale_price, current_stock, weight_grams, status, production_date, expiration_date, entry_date) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&product.name)
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

    pub async fn find_by_id(pool: &MySqlPool, id: i32) -> Result<Option<Product>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, name, cost_price, sale_price, current_stock, weight_grams, status, production_date, expiration_date, entry_date FROM products WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(Self::row_to_product))
    }

    pub async fn list(pool: &MySqlPool) -> Result<Vec<Product>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, name, cost_price, sale_price, current_stock, weight_grams, status, production_date, expiration_date, entry_date FROM products"
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(Self::row_to_product).collect())
    }

    pub async fn update(pool: &MySqlPool, product: &Product) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE products SET name = ?, cost_price = ?, sale_price = ?, current_stock = ?, weight_grams = ?, status = ?, production_date = ?, expiration_date = ?, entry_date = ? WHERE id = ?",
        )
        .bind(&product.name)
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

    /// Busca lotes de um produto ordenados por entry_date ASC (FIFO).
    pub async fn find_batches_by_name(pool: &MySqlPool, product_name: &str) -> Result<Vec<Product>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, name, cost_price, sale_price, current_stock, weight_grams, status, production_date, expiration_date, entry_date FROM products WHERE name = ? ORDER BY entry_date ASC"
        )
        .bind(product_name)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(Self::row_to_product).collect())
    }

    pub async fn update_quantity(pool: &MySqlPool, product_id: i32, new_quantity: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE products SET current_stock = ? WHERE id = ?")
            .bind(new_quantity)
            .bind(product_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_min_stock(pool: &MySqlPool, product_id: i32) -> Result<i32, sqlx::Error> {
        let row = sqlx::query("SELECT min_stock FROM products WHERE id = ?")
            .bind(product_id)
            .fetch_one(pool)
            .await?;
        Ok(row.get("min_stock"))
    }

    /// Relatório completo de estoque agrupado por produto.
    pub async fn stock_report(pool: &MySqlPool) -> Result<Vec<StockReport>, sqlx::Error> {
        let products = Self::list(pool).await?;
        let mut reports = Vec::new();

        for product in products {
            let batches = Self::find_batches_by_name(pool, &product.name).await?;
            let total_stock: i32 = batches.iter().map(|b| b.current_stock).sum();

            reports.push(StockReport {
                product_id: product.id,
                product_name: product.name,
                status: product.status,
                total_stock,
                batches: batches.into_iter().map(|b| BatchReport {
                    id: b.id,
                    quantity: b.current_stock,
                    entry_date: b.entry_date,
                }).collect(),
            });
        }

        Ok(reports)
    }

    /// Produtos com estoque crítico (total ≤ 5 unidades).
    pub async fn critical_stock(pool: &MySqlPool) -> Result<Vec<StockReport>, sqlx::Error> {
        let reports = Self::stock_report(pool).await?;
        Ok(reports.into_iter().filter(|p| p.total_stock <= 5).collect())
    }
}
