use sqlx::FromRow;
use serde::{Serialize, Deserialize};

/// Representa um cargo/papel no sistema de permissões.
///
/// Os cargos definem o nível de acesso do usuário:
/// - Admin (nível 3): acesso total
/// - Gerente (nível 2): acesso intermediário
/// - Funcionário (nível 1): acesso básico
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

impl Role {
    /// Retorna o nível numérico de permissão associado ao cargo.
    pub fn nivel(&self) -> i16 {
        match self.name.as_str() {
            "Admin" => 3,
            "Gerente" => 2,
            "Funcionario" => 1,
            _ => 0,
        }
    }
}
