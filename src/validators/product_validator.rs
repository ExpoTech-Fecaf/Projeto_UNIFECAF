use chrono::NaiveDate;
use sqlx::MySqlPool;
use crate::validators::user_validator::ValidationError;

/// Validador de dados de produto.
///
/// Garante integridade dos dados antes da persistência:
/// nome único, status válido e datas no formato correto.
pub struct ProductValidator;

impl ProductValidator {
    /// Valida se o nome do produto é único no banco (case-insensitive).
    pub async fn validate_name_unique(pool: &MySqlPool, name: &str) -> Result<(), ValidationError> {
        Self::validate_name_unique_excluding(pool, name, None).await
    }

    /// Valida se o nome do produto é único, excluindo um ID específico (para updates).
    pub async fn validate_name_unique_excluding(pool: &MySqlPool, name: &str, exclude_id: Option<i32>) -> Result<(), ValidationError> {
        let name = name.trim();

        if name.is_empty() {
            return Err(ValidationError::new("name", "O nome do produto não pode ficar em branco"));
        }

        let result = match exclude_id {
            Some(id) => sqlx::query("SELECT id FROM products WHERE LOWER(name) = LOWER(?) AND id != ?")
                .bind(name)
                .bind(id)
                .fetch_optional(pool)
                .await,
            None => sqlx::query("SELECT id FROM products WHERE LOWER(name) = LOWER(?)")
                .bind(name)
                .fetch_optional(pool)
                .await,
        };

        match result {
            Ok(Some(_)) => Err(ValidationError::new(
                "name",
                "Já existe um produto com este nome",
            )),
            Ok(None) => Ok(()),
            Err(_) => Err(ValidationError::new(
                "name",
                "Erro ao validar nome do produto",
            )),
        }
    }

    /// Valida se o status é válido (1 = ativo, 2 = inativo).
    pub fn validate_status(status: i16) -> Result<i16, ValidationError> {
        match status {
            1 | 2 => Ok(status),
            _ => Err(ValidationError::new(
                "status",
                "Status inválido. Use 1 para ativo ou 2 para inativo",
            )),
        }
    }

    /// Valida e converte uma string de data no formato `dd/mm/YYYY` para `NaiveDate`.
    pub fn validate_and_parse_date(date_str: &str, field: &str) -> Result<NaiveDate, ValidationError> {
        let date_str = date_str.trim();

        match NaiveDate::parse_from_str(date_str, "%d/%m/%Y") {
            Ok(date) => Ok(date),
            Err(_) => Err(ValidationError::new(
                field,
                "Data inválida. Use o formato dd/mm/YYYY",
            )),
        }
    }
}
