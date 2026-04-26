use chrono::NaiveDate;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Usuario {
    pub id: i32,
    pub nome: String,
    pub sobrenome: String,
    pub cpf: String,
    pub data_nascimento: NaiveDate,
    pub user: String,
    pub senha: String,
    pub fkidcargo: i32,
}