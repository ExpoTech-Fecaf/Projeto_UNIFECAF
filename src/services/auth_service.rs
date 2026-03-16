use crate::models::user::{User, UserType}; // Importações e uso do crates
use bcrypt::{hash, verify, DEFAULT_COST}; // Importa as funções hash e verify

use std::error::Error; // Importa o trait de Error da biblioteca padrão

// Função para criptografar a senha
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

// Função para verificar uma senha
pub fn verify_password(hash: &str, password: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(hash, password)
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
    match (user.user_type.clone(), required_type) {
        (UserType::Admin, _) => true, // Admin tem permissão para tudo
        (UserType::Gerente, UserType::Funcionario) => true, // Gerente tem permissão para Funcionári
        (UserType::Gerente, UserType::Gerente) => true, // Gerente tem permissão para Gerente
        (UserType::Funcionario, UserType::Funcionario) => true, // Funcionário tem permissão para Funcionario
        _ => false,
    }
}