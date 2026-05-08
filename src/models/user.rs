// Biblioteca para converter os dados em formato Json
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

/// Enum que representa os tipos de usuários do sistema.
///
/// Cada variante corresponde a um nível diferente de permissão,
/// utilizado pelo sistema de controle de acesso.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
pub enum UserType {
    /// Usuário administrador — acesso total (nível 3)
    Admin,
    /// Usuário gerente — acesso intermediário (nível 2)
    Gerente,
    /// Usuário funcionário — acesso básico (nível 1)
    Funcionario,
}

impl UserType {
    /// Retorna o nível numérico de permissão do tipo de usuário.
    ///
    /// Utilizado para comparação hierárquica de permissões.
    pub fn nivel(&self) -> i16 {
        match self {
            UserType::Admin => 3,
            UserType::Gerente => 2,
            UserType::Funcionario => 1,
        }
    }
}

/// Struct que representa um usuário do sistema.
///
/// Armazena informações para autenticação, identificação pessoal
/// e controle de permissão baseado em cargo.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    /// ID do usuário (None para usuários ainda não persistidos)
    pub id: Option<i32>,
    /// Nome de usuário para login (único)
    pub username: String,
    /// Hash bcrypt da senha
    pub password_hash: String,
    /// Tipo/nível de permissão
    pub user_type: UserType,
    /// Primeiro nome
    pub first_name: String,
    /// Sobrenome
    pub last_name: String,
    /// Data de nascimento
    pub birth_date: NaiveDate,
    /// CPF (11 dígitos, validado)
    pub cpf: String,
    /// FK para a tabela de cargos
    pub role_id: i16,
}

impl User {
    /// Cria um novo usuário com `id = None` (ainda não persistido no banco).
    pub fn new(
        username: String,
        password_hash: String,
        user_type: UserType,
        first_name: String,
        last_name: String,
        birth_date: NaiveDate,
        cpf: String,
        role_id: i16,
    ) -> Self {
        Self {
            id: None,
            username,
            password_hash,
            user_type,
            first_name,
            last_name,
            birth_date,
            cpf,
            role_id,
        }
    }
}