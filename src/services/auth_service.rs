use crate::models::user::{User, UserType};
use bcrypt::{hash, verify, DEFAULT_COST};

/// Gera o hash bcrypt de uma senha em texto plano.
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

/// Verifica se a senha fornecida corresponde ao hash bcrypt armazenado.
pub fn verify_password(password: &str, stored: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, stored)
}

/// Autentica um usuário buscando pelo username e verificando a senha.
///
/// # Erros
/// - `"User not found"` se o username não existir
/// - `"Password incorrect"` se a senha não corresponder
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

/// Verifica se o usuário possui permissão igual ou superior ao tipo requerido.
///
/// Compara o nível numérico do usuário com o nível mínimo exigido.
pub fn check_permission(user: &User, required_type: UserType) -> bool {
    user.user_type.nivel() >= required_type.nivel()
}
