use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

/// Enum que representa o tipo de movimentação de estoque.
#[derive(Debug, Serialize, Deserialize)]
pub enum MovementType {
    /// Entrada de produtos no estoque
    #[serde(rename = "entrada")]
    Entrada,
    /// Saída de produtos do estoque
    #[serde(rename = "saida")]
    Saida,
}

/// Registro de movimentação de estoque (entrada ou saída).
///
/// Cada movimentação é vinculada a um produto, opcionalmente a um lote específico,
/// e ao usuário que realizou a operação.
#[derive(Debug, Serialize, Deserialize)]
pub struct Movement {
    /// ID da movimentação (None para registros ainda não persistidos)
    pub id: Option<i32>,
    /// ID do produto associado
    pub product_id: i32,
    /// ID do lote específico (quando aplicável)
    pub batch_id: Option<i32>,
    /// ID do usuário que realizou a movimentação
    pub user_id: i32,
    /// Tipo: "entrada" ou "saida"
    pub movement_type: String,
    /// Quantidade movimentada
    pub quantity: i32,
    /// Data/hora da movimentação (preenchido automaticamente pelo banco)
    pub created_at: Option<NaiveDateTime>,
    /// Observações opcionais
    pub notes: Option<String>,
}
