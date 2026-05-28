use chrono::NaiveDate;
use gerenciamento_de_estoque::models::user::UserType;
use gerenciamento_de_estoque::validators::user_validator::UserValidator;
use gerenciamento_de_estoque::validators::product_validator::ProductValidator;

#[test]
fn test_user_type_nivel() {
    assert_eq!(UserType::Admin.nivel(), 3);
    assert_eq!(UserType::Gerente.nivel(), 2);
    assert_eq!(UserType::Funcionario.nivel(), 1);
}

#[test]
fn test_validate_role_id() {
    assert!(UserValidator::validate_role_id(1).is_ok());
    assert!(UserValidator::validate_role_id(2).is_ok());
    assert!(UserValidator::validate_role_id(3).is_ok());
    assert!(UserValidator::validate_role_id(99).is_err());
}

#[test]
fn test_validate_cpf_examples() {
    // CPF de exemplo válido (529.982.247-25)
    assert!(UserValidator::validate_cpf("52998224725").is_ok());

    // CPF com tamanho incorreto
    assert!(UserValidator::validate_cpf("123").is_err());

    // CPF com todos dígitos repetidos
    assert!(UserValidator::validate_cpf("11111111111").is_err());

    // CPF com dígito verificador inválido
    assert!(UserValidator::validate_cpf("52998224724").is_err());
}

#[test]
fn test_user_validate_and_parse_date() {
    let ok = UserValidator::validate_and_parse_date("13/12/2006");
    assert!(ok.is_ok());
    assert_eq!(ok.unwrap(), NaiveDate::from_ymd_opt(2006, 12, 13).unwrap());

    // Formato inválido
    assert!(UserValidator::validate_and_parse_date("2006-12-13").is_err());

    // Data no futuro
    assert!(UserValidator::validate_and_parse_date("01/01/3000").is_err());
}

#[test]
fn test_product_validator_status_and_date() {
    assert_eq!(ProductValidator::validate_status(1).unwrap(), 1);
    assert_eq!(ProductValidator::validate_status(2).unwrap(), 2);
    assert!(ProductValidator::validate_status(0).is_err());

    // Data válida
    let d = ProductValidator::validate_and_parse_date("10/10/2020", "expiry").unwrap();
    assert_eq!(d, NaiveDate::from_ymd_opt(2020, 10, 10).unwrap());

    // Formato inválido
    assert!(ProductValidator::validate_and_parse_date("2020-10-10", "expiry").is_err());
}
