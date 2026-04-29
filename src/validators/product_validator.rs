use chrono::NaiveDate;
use sqlx::MySqlPool;
use crate::validators::user_validator::ValidationError;

pub struct ProductValidator;

impl ProductValidator {
    pub async fn validate_name_unique(pool: &MySqlPool, name: &str) -> Result<(), ValidationError> {
        let name = name.trim();

        if name.is_empty() {
            return Err(ValidationError::new("name", "O nome do produto não pode ficar em branco"));
        }

        let result = sqlx::query("SELECT id FROM produto WHERE LOWER(nome) = LOWER(?)")
            .bind(name)
            .fetch_optional(pool)
            .await;

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

    pub fn validate_status(status: i16) -> Result<i16, ValidationError> {
        match status {
            1 | 2 => Ok(status),
            _ => Err(ValidationError::new(
                "status",
                "Status inválido. Use 1 para ativo ou 2 para inativo",
            )),
        }
    }

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
