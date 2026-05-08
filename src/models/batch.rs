use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

/// Representa um lote físico de produto no estoque.
///
/// Utilizado para rastrear quantidades individuais por data de entrada,
/// permitindo a aplicação da lógica FIFO na retirada.
#[derive(Debug, Serialize, Deserialize)]
pub struct Batch {
    /// ID do lote (None para registros ainda não persistidos)
    pub id: Option<i32>,
    /// ID do produto ao qual este lote pertence
    pub product_id: i32,
    /// Quantidade disponível neste lote
    pub quantity: i32,
    /// Data de entrada no estoque — ordenação ASC para FIFO
    pub entry_date: NaiveDate,
}