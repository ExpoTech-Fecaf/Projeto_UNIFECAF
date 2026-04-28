use chrono::NaiveDate;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Produto {
    pub id: i32,
    pub nome: String,
    pub valor_custo: f64,
    pub valor_venda: f64,
    pub estoque_atual: i32,
    pub peso_gramas: i32,
    pub status: i16, // 1 ou 2
    pub data_producao: NaiveDate,
    pub data_validade: NaiveDate,
}