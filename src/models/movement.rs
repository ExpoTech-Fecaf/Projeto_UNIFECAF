use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MovementType {
    #[serde(rename = "entrada")]
    Entrada,
    #[serde(rename = "saida")]
    Saida,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Movement {
    pub id: Option<i32>,
    pub product_id: i32,
    pub batch_id: Option<i32>,
    pub user_id: i32,
    pub movement_type: String, // "entrada" ou "saida"
    pub quantity: i32,
    pub created_at: Option<NaiveDateTime>,
    pub notes: Option<String>,
}
