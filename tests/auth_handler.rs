use chrono::NaiveDate;
use gerenciamento_de_estoque::models::user::{User, UserType};
use gerenciamento_de_estoque::services::auth_service;

#[test]
fn test_authenticate_user() {
    let password = "admin321";
    let hashed_password = auth_service::hash_password(password).unwrap();

    let users = vec![
        User{
            id: None,
            username: "admin".to_string(),
            password_hash: hashed_password,
            user_type: UserType::Admin,
            first_name: "admin".to_string(),
            last_name: "321".to_string(),
            birth_date: NaiveDate::from_ymd_opt(2006, 12, 13).unwrap(),
            cpf: "00000000000".to_string(),
            role_id: 1,
        }
    ];

    // Teste com credenciais corretas
    let result = auth_service::authenticate_user(&users, "admin", "admin321");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().user_type, UserType::Admin);

    // Teste com senha incorreta
    let result = auth_service::authenticate_user(&users, "admin", "wrong_password");
    assert!(result.is_err());

    // Teste com usuário incorreto
    let result = auth_service::authenticate_user(&users, "nonexists", "admin321");
    assert!(result.is_err());
}