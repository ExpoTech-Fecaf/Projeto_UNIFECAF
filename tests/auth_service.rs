use chrono::NaiveDate;
use gerenciamento_de_estoque::models::user::{User, UserType};
use gerenciamento_de_estoque::services::auth_service;

// Teste para a função hash_password
#[test]
fn test_hash_password() {
    let password = "minha_senha"; // Gerando a senha
    let hashed_password = auth_service::hash_password(password).unwrap(); // Transformando a senha em hash
    assert_ne!(password, hashed_password); // Validando se a senha são diferentes
}

// Teste para a função verify_password
#[test]
fn test_verify_password() {
    let password = "minha_senha"; // Gerando a senha
    let hashed_password = auth_service::hash_password(password).unwrap(); // Transformando a senha em hash

    assert!(auth_service::verify_password(password, &hashed_password).unwrap()); // Verifica se o hash gerado corresponde à senha original
}

// Teste para a função de autenticar usuário
#[test]
fn test_authenticate_user(){
    let password = "minha_senha"; // Gerando a senha
    let hashed_password = auth_service::hash_password(password).unwrap(); // Transformando a senha em hash

    // Criando um novo user
    let user = vec![
        User{
            id: None,
            username: "icaro".to_string(),
            password_hash: hashed_password,
            user_type: UserType::Admin,
            first_name: "Icaro".to_string(),
            last_name: "Rodrigues".to_string(),
            birth_date: NaiveDate::from_ymd_opt(2006, 12, 13).unwrap(),
        }
    ];

    // Teste com credencias corretas
    assert!(auth_service::authenticate_user(&user, "icaro", "minha_senha").is_ok());
    // Teste com a senha incorreta
    assert!(auth_service::authenticate_user(&user, "Icaro", "senha_incorreta").is_err());
    // Teste com usuário não encontrado
    assert!(auth_service::authenticate_user(&user, "não existe", "minha_senha").is_err());
}

// Teste para a função check_permission
#[test]
fn test_check_permission(){
    // Criando um admin
    let admin = User {
        id: None,
        username: "Admin".to_string(),
        password_hash: "hash".to_string(),
        user_type: UserType::Admin,
        first_name: "admin".to_string(),
        last_name: "123".to_string(),
        birth_date: NaiveDate::from_ymd_opt(2017, 01, 01).unwrap(),
    };
    // Criando um gerente
    let gerente = User {
        id: None,
        username: "Gerente".to_string(),
        password_hash: "hash".to_string(),
        user_type: UserType::Gerente,
        first_name: "Icaro".to_string(),
        last_name: "Rodrigues".to_string(),
        birth_date: NaiveDate::from_ymd_opt(2006, 12, 13).unwrap(),
    };
    // Criando um funcionário
    let funcionario = User {
        id: None,
        username: "Funcionario".to_string(),
        password_hash: "hash".to_string(),
        user_type: UserType::Funcionario,
        first_name: "Rafael".to_string(),
        last_name: "Celestino".to_string(),
        birth_date: NaiveDate::from_ymd_opt(2006, 08, 21).unwrap(),
    };

    // Admin tem permissão para tudo
    assert!(auth_service::check_permission(&admin, UserType::Admin));
    assert!(auth_service::check_permission(&admin, UserType::Gerente));
    assert!(auth_service::check_permission(&admin, UserType::Funcionario));
    // Gerente tem permissão para funcionário e gerente
    assert!(auth_service::check_permission(&gerente, UserType::Funcionario));
    assert!(auth_service::check_permission(&gerente, UserType::Gerente));
    assert!(!auth_service::check_permission(&gerente, UserType::Admin));
    // Funcionário só tem permissão para funcionário
    assert!(auth_service::check_permission(&funcionario, UserType::Funcionario));
    assert!(!auth_service::check_permission(&funcionario, UserType::Gerente));
    assert!(!auth_service::check_permission(&funcionario, UserType::Admin));

}


