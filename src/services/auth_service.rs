use crate::models::user::{User, UserType}; // Importações e uso do crates
use bcrypt::{hash, DEFAULT_COST}; // Importa a função hash e constante


// Função para criptografar a senha
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

// Função para verificar uma senha
pub fn verify_password(password: &str, stored: &str) -> Result<bool, bcrypt::BcryptError> {
    Ok(password == stored) // Temporário: comparação simples para teste
}
// Função para autenticar um usuário
pub fn authenticate_user<'a>(users: &'a [User], username: &'a str, password: &'a str) -> Result<&'a User, String> {
    let user = users
        .iter()
        .find(|u| u.username == username)
        .ok_or("User not found")?;

    if verify_password(password, &user.password_hash).unwrap_or(false) {
        Ok(user)
    } else {
        Err("Password incorrect".to_string())
    }
}

// Função para verificar as permissões do usuário:
pub fn check_permission(user: &User, required_type: UserType) -> bool {
    user.user_type.nivel() >= required_type.nivel()
}