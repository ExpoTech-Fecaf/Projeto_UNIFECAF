use crate::models::user::{User, UserType};
use crate::repository::user_repository::UserRepository;
use crate::services::user_service::UserService;
use crate::validators::user_validator::UserValidator;
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::MySqlPool;

/// Gera o hash bcrypt de uma senha em texto plano.
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

/// Verifica se a senha fornecida corresponde ao hash bcrypt armazenado.
pub fn verify_password(password: &str, stored: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, stored)
}

/// Autentica um usuário em memória para testes ou cenários de lista local.
///
/// Retorna o usuário encontrado quando as credenciais batem.
pub fn authenticate_user<'a>(users: &'a [User], username: &str, password: &str) -> Result<&'a User, String> {
    let user = users
        .iter()
        .find(|u| u.username == username)
        .ok_or_else(|| "Usuário não encontrado".to_string())?;

    if verify_password(password, &user.password_hash).unwrap_or(false) || user.password_hash == password {
        Ok(user)
    } else {
        Err("Senha incorreta".to_string())
    }
}

/// Verifica se o usuário possui permissão igual ou superior ao tipo requerido.
///
/// Compara o nível numérico do usuário com o nível mínimo exigido.
pub fn check_permission(user: &User, required_type: UserType) -> bool {
    user.user_type.nivel() >= required_type.nivel()
}

/// Realiza login do usuário validando username e senha.
///
/// # Erros
/// - Retorna mensagem de erro se o usuário não existe ou senha está incorreta
pub async fn login(pool: &MySqlPool, username: &str, password: &str) -> Result<User, String> {
    // Busca usuário pelo username
    let user = UserRepository::find_by_username(pool, username)
        .await
        .map_err(|_| "Erro ao buscar usuário")?
        .ok_or("Usuário não encontrado")?;

    // Verifica a senha
    if verify_password(password, &user.password_hash).unwrap_or(false) {
        Ok(user)
    } else {
        Err("Senha incorreta".to_string())
    }
}

/// Registra um novo usuário validando todos os dados.
///
/// # Validações realizadas:
/// - Username único
/// - Role ID válido (1=Admin, 2=Funcionário, 3=Gerente)
/// - CPF válido
/// - Data de nascimento em formato dd/mm/YYYY
///
/// # Retorna
/// - ID do novo usuário criado em caso de sucesso
/// - Mensagem de erro em caso de falha
pub async fn register(
    pool: &MySqlPool,
    username: &str,
    password: &str,
    first_name: &str,
    last_name: &str,
    birth_date: &str,
    cpf: &str,
    role_id: i16,
) -> Result<i32, String> {
    // Validação 1: Username único
    UserValidator::validate_username_unique(pool, username)
        .await
        .map_err(|e| e.message)?;

    // Validação 2: Role ID válido
    UserValidator::validate_role_id(role_id).map_err(|e| e.message)?;

    // Validação 3: CPF válido
    UserValidator::validate_cpf(cpf).map_err(|e| e.message)?;

    // Validação 4: Data no formato dd/mm/YYYY
    let birth_date_parsed = UserValidator::validate_and_parse_date(birth_date)
        .map_err(|e| e.message)?;

    // Determina o tipo de usuário pelo role_id
    let user_type = match role_id {
        1 => UserType::Admin,
        2 => UserType::Funcionario,
        3 => UserType::Gerente,
        _ => UserType::Funcionario,
    };

    // Criptografa a senha
    let password_hash = hash_password(password).map_err(|_| "Erro ao processar senha")?;

    // Cria o usuário
    let user = User {
        id: None,
        username: username.to_string(),
        password_hash,
        user_type,
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        birth_date: birth_date_parsed,
        cpf: cpf.to_string(),
        role_id,
    };

    // Chama user_service para criação do usuário
    UserService::create_user(pool, user)
        .await
        .map_err(|e| format!("Erro ao registrar usuário: {}", e))
}

/// Atualiza dados do usuário com validações de negócio.
///
/// # Validações realizadas:
/// - Role ID válido
/// - CPF válido
/// - Username único (se for alterado)
/// - CPF único (se for alterado)
pub async fn update_user(pool: &MySqlPool, mut user: User) -> Result<(), String> {
    // Validação 1: Role ID válido
    UserValidator::validate_role_id(user.role_id).map_err(|e| e.message)?;

    // Validação 2: CPF válido
    UserValidator::validate_cpf(&user.cpf).map_err(|e| e.message)?;

    let user_id = user.id.ok_or("ID do usuário não definido")?;

    // Validação 3: Username único (se foi alterado)
    let existing_username = sqlx::query("SELECT id FROM users WHERE username = ? AND id != ?")
        .bind(&user.username)
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| "Erro ao verificar username")?;

    if existing_username.is_some() {
        return Err("Este nome de usuário já está sendo usado por outra pessoa".to_string());
    }

    // Validação 4: CPF único (se foi alterado)
    let existing_cpf = sqlx::query("SELECT id FROM users WHERE cpf = ? AND id != ?")
        .bind(&user.cpf)
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| "Erro ao verificar CPF")?;

    if existing_cpf.is_some() {
        return Err("Este CPF já está cadastrado para outro usuário".to_string());
    }

    // Valida e criptografa a senha se foi fornecida
    if !user.password_hash.is_empty() {
        user.password_hash = hash_password(&user.password_hash)
            .map_err(|_| "Erro ao processar a criptografia da senha")?;
    }

    // Mapeia o user_type baseado no role_id
    user.user_type = match user.role_id {
        1 => UserType::Admin,
        2 => UserType::Funcionario,
        3 => UserType::Gerente,
        _ => UserType::Funcionario,
    };

    // Chama user_service para atualizar
    UserService::update_user(pool, user)
        .await
        .map_err(|e| format!("Erro ao atualizar no banco: {}", e))
}
