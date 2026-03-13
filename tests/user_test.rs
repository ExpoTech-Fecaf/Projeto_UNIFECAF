// Teste do modelo user
use gerenciamento_de_estoque::models::{User, UserType};
use bcrypt::{hash, DEFAULT_COST};

#[test]
fn test_user_creation() {
    let plain_password = "123455";

    let hashed = hash(plain_password, DEFAULT_COST)
        .expect("hashing failed");

    let user = User::new (
        "Icaro".to_string(),
        hashed,
        UserType::Admin,
        );

    println!("user created: {:?}", user);
}