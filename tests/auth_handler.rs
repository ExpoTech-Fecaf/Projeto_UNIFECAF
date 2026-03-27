use chrono::NaiveDate;
use gerenciamento_de_estoque::models::user::{User, UserType};
use gerenciamento_de_estoque::services::auth_service;
use gerenciamento_de_estoque::handlers::auth_handler;

#[test]
fn test_login() {
    let password = "admin321";
    let hashed_password = auth_service::hash_password(password);

    let users = vec![
        User{
            id: None,
            username: "admin".to_string(),
            password_hash: hashed_password.unwrap(),
            user_type: UserType::Admin,
            first_name: "admin".to_string(),
            last_name: "321".to_string(),
            birth_date: NaiveDate::from_ymd_opt(2006, 12, 13).unwrap(),
        }
    ];

    // Teste com credencias corretas
    let response = auth_handler::login(&users, "admin", "admin321");
    assert!(response.success);
    assert_eq!(response.user_type, Some(UserType::Admin));

    // Teste com senha incorreta
    let response = auth_handler::login(&users, "admin", "wrong_password");
    assert!(!response.success);
    assert_eq!(response.user_type, None);

    // Teste com usuário incorreto
    let response = auth_handler::login(&users, "nonexists", "admin321");
    assert!(!response.success);
    assert_eq!(response.user_type, None);
}