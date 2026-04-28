use chrono::NaiveDate;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub cost_price: f64,
    pub sale_price: f64,
    pub current_stock: i32,
    pub weight_grams: i32,
    pub status: i16, // 1 or 2
    pub production_date: NaiveDate,
    pub expiration_date: NaiveDate,
}