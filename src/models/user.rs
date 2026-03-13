// Biblioteca para converter os dados em formato Json
use serde::{Serialize, Deserialize};

// Enum que representa os tipos de usuários do sistema.
// Cada variante corresponde a um nivel diferente de permissão.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserType {
    Admin, // Usuário administrador (acesso total)
    Gerente, // Usuário gerente (acesso intermediário)
    Funcionario // Usuário funcionário (acesso básico)
}

// Struct que representa os usuários do sistema.
// Armazena informações essenciais para a autenticação e controle de permissão
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password_hash: String,
    pub user_type: UserType,
}

impl User {
    // Cria o novo usuário com os dados fornecidos
    // O id é definido como 'None' pois novos usuários ainda não tem um id definido no banco de dados.
    pub fn new(username: String, password_hash: String, user_type: UserType) -> User {
        Self {
            id: None,
            username,
            password_hash,
            user_type,
        }
    }
}