use chrono::NaiveDate;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

/// Representa um produto/lote no sistema de estoque.
///
/// Cada registro corresponde a um lote individual. Produtos com o mesmo `name`
/// são tratados como lotes distintos, ordenados por `entry_date` para a lógica FIFO.
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Product {
    /// Identificador único do lote no banco de dados
    pub id: i32,
    /// Nome do produto (ex: "Arroz", "Feijão")
    pub name: String,
    /// Preço de custo unitário
    pub cost_price: f64,
    /// Preço de venda unitário
    pub sale_price: f64,
    /// Quantidade atual em estoque neste lote
    pub current_stock: i32,
    /// Peso em gramas por unidade
    pub weight_grams: i32,
    /// Status do produto: 1 = ativo, 2 = inativo
    pub status: i16,
    /// Data de produção do lote
    pub production_date: NaiveDate,
    /// Data de validade do lote
    pub expiration_date: NaiveDate,
    /// Data de entrada no estoque — campo essencial para ordenação FIFO
    pub entry_date: NaiveDate,
}