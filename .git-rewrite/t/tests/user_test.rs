// Teste do modelo user
use gerenciamento_de_estoque::models::{User, UserType};
use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDate;

#[test]
fn test_user_creation() {
    let plain_password = "123455";

    let hashed = hash(plain_password, DEFAULT_COST)
        .expect("hashing failed");

    let user = User::new (
        "Icaro".to_string(),
        hashed,
        UserType::Admin,
        "Icaro".to_string(),
        "Rodrigues".to_string(),
        NaiveDate::from_ymd_opt(2006, 12, 13).unwrap(),
        );

    println!("user created: {:?}", user);
}