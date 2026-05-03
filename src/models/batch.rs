use serde::{Deserialize, Serialize};
use chrono::NaiveDate; // Importante para a lógica de data FIFO

#[derive(Debug, Serialize, Deserialize)]
pub struct Batch {
    pub id: Option<i32>,
    pub product_id: i32,
    pub quantity: i32,
    pub entry_date: NaiveDate, // Campo essencial para ordenar os itens mais antigos primeiro
}