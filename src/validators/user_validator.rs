use chrono::NaiveDate;
use sqlx::MySqlPool;

/// Estrutura para erros de validação de usuário
#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: &str, message: &str) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}

/// Validador de usuários
pub struct UserValidator;

impl UserValidator {
    /// Valida se o username é único no banco de dados
    pub async fn validate_username_unique(pool: &MySqlPool, username: &str) -> Result<(), ValidationError> {
        let result = sqlx::query("SELECT id FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(pool)
            .await;

        match result {
            Ok(Some(_)) => Err(ValidationError::new(
                "username",
                "Usuário com este username já existe",
            )),
            Ok(None) => Ok(()),
            Err(_) => Err(ValidationError::new(
                "username",
                "Erro ao validar username",
            )),
        }
    }

    /// Valida se o role_id é válido (1=Admin, 2=Funcionário, 3=Gerente)
    pub fn validate_role_id(role_id: i16) -> Result<(), ValidationError> {
        match role_id {
            1 | 2 | 3 => Ok(()),
            _ => Err(ValidationError::new(
                "role_id",
                "Role ID inválido. Deve ser: 1 (Admin), 2 (Funcionário) ou 3 (Gerente)",
            )),
        }
    }

    /// Valida se o CPF é válido (sem números repetidos e formato correto)
    pub fn validate_cpf(cpf: &str) -> Result<(), ValidationError> {
        // Remove caracteres especiais
        let clean_cpf: String = cpf.chars().filter(|c| c.is_numeric()).collect();

        // Verifica se tem 11 dígitos
        if clean_cpf.len() != 11 {
            return Err(ValidationError::new(
                "cpf",
                "CPF deve conter 11 dígitos",
            ));
        }

        // Verifica se não tem todos os números iguais
        let first_digit = clean_cpf.chars().next().unwrap();
        if clean_cpf.chars().all(|c| c == first_digit) {
            return Err(ValidationError::new(
                "cpf",
                "CPF com números repetidos não é válido",
            ));
        }

        // Validação do primeiro dígito verificador
        let first_check = Self::calculate_cpf_check_digit(&clean_cpf[0..9], 10);
        if first_check != clean_cpf.chars().nth(9).unwrap().to_digit(10).unwrap() as u32 {
            return Err(ValidationError::new(
                "cpf",
                "CPF inválido (falha na validação do primeiro dígito)",
            ));
        }

        // Validação do segundo dígito verificador
        let second_check = Self::calculate_cpf_check_digit(&clean_cpf[0..10], 11);
        if second_check != clean_cpf.chars().nth(10).unwrap().to_digit(10).unwrap() as u32 {
            return Err(ValidationError::new(
                "cpf",
                "CPF inválido (falha na validação do segundo dígito)",
            ));
        }

        Ok(())
    }

    /// Calcula o dígito verificador do CPF
    fn calculate_cpf_check_digit(partial_cpf: &str, multiplier: u32) -> u32 {
        let sum: u32 = partial_cpf
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let digit = c.to_digit(10).unwrap_or(0);
                digit * (multiplier - i as u32)
            })
            .sum();

        let remainder = sum % 11;
        if remainder < 2 {
            0
        } else {
            11 - remainder
        }
    }

    /// Valida e converte data no formato dd/mm/YYYY para NaiveDate
    pub fn validate_and_parse_date(date_str: &str) -> Result<NaiveDate, ValidationError> {
        // Remove espaços em branco
        let date_str = date_str.trim();

        // Tenta fazer parse no formato dd/mm/YYYY
        match NaiveDate::parse_from_str(date_str, "%d/%m/%Y") {
            Ok(date) => {
                // Valida se a data não é no futuro
                if date > chrono::Local::now().naive_local().date() {
                    Err(ValidationError::new(
                        "birth_date",
                        "Data de nascimento não pode ser no futuro",
                    ))
                } else {
                    Ok(date)
                }
            }
            Err(_) => Err(ValidationError::new(
                "birth_date",
                "Data de nascimento inválida. Use o formato dd/mm/YYYY",
            )),
        }
    }
}
