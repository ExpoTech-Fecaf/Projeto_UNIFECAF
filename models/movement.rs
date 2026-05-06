use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipoMovimento {
    Entrada,
    Saida,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movement {
    pub tipo_movimento: TipoMovimento,
    pub produto_id: i32,
    pub quantidade: i32,
    pub usuario_id: i32,
    pub data_hora: NaiveDateTime,
}