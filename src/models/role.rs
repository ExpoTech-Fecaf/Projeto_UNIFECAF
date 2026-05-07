use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

impl Role {
    pub fn nivel(&self) -> i16 {
        match self.name.as_str() {
            "Admin" => 3,
            "Gerente" => 2,
            "Funcionario" => 1,
            _ => 0, // Retorna 0 para roles desconhecidos
        }
    }
}