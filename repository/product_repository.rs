use sqlx::MySqlPool;
use sqlx::Row;
use crate::models::product::Product;
use diesel::prelude::*;
use crate::models::product::Product;
use crate::schema::products; 

pub struct ProductRepository;

impl ProductRepository {
    pub async fn create(pool: &MySqlPool, product: &Product) -> Result<i32, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO produto (nome, valorcusto, valorvenda, estoqueatual, pesogramas, status, dataproducao, datavalidade, data_entrada)
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
            SELECT id, nome, valorcusto, valorvenda, estoqueatual, pesogramas, status, dataproducao, datavalidade, data_entrada
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
                entry_date: row.get("data_entrada"),
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
                pesogramas = ?, status = ?, dataproducao = ?, datavalidade = ?, data_entrada = ?
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
        sqlx::query("DELETE FROM produto WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// Busca todos os lotes (produtos) de um tipo específico, ordenados por entry_date (ASC) para FIFO
    pub async fn find_batches_by_name(pool: &MySqlPool, product_name: &str) -> Result<Vec<Product>, sqlx::Error> {
        let batches = sqlx::query(
            r#"
            SELECT id, nome, valorcusto, valorvenda, estoqueatual, pesogramas, status, dataproducao, datavalidade, data_entrada
            FROM produto
            WHERE nome = ?
            ORDER BY data_entrada ASC
            "#
        )
        .bind(product_name)
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
                entry_date: row.get("data_entrada"),
            }
        })
        .collect();

        Ok(batches)
    }

    /// Atualiza apenas a quantidade de estoque de um lote específico
    pub async fn update_quantity(pool: &MySqlPool, product_id: i32, new_quantity: i32) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE produto SET estoqueatual = ? WHERE id = ?")
            .bind(new_quantity)
            .bind(product_id)
            .execute(pool)
            .await?;
        Ok(())
    }

pub trait ProductRepository {
    fn buscar_estoque_total(&self, produto_id: i32) -> Result<i32, diesel::result::Error>;
}

pub struct DieselProductRepository;

impl ProductRepository for DieselProductRepository {
    fn buscar_estoque_total(&self, produto_id: i32) -> Result<i32, diesel::result::Error> {
        use crate::schema::products::dsl::*;
        let conn = &mut establish_connection(); // Função que retorna PgConnection
        products
            .filter(id.eq(produto_id))
            .select(quantidade) // Assumindo que a coluna se chama "quantidade"
            .first::<i32>(conn)
    }
}
}
