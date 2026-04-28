use axum::extract::{State, Json};
use sqlx::MySqlPool;
use serde_json::json;
use chrono::NaiveDate;
use crate::models::product::Product;
use crate::services::product_service::ProductService;

#[derive(serde::Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub cost_price: f64,
    pub sale_price: f64,
    pub current_stock: i32,
    pub weight_grams: i32,
    pub status: i16,
    pub production_date: String,
    pub expiration_date: String,
}

pub async fn create_product(
    State(pool): State<MySqlPool>,
    Json(req): Json<CreateProductRequest>,
) -> Json<serde_json::Value> {
    let production_date = NaiveDate::parse_from_str(&req.production_date, "%Y-%m-%d").unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
    let expiration_date = NaiveDate::parse_from_str(&req.expiration_date, "%Y-%m-%d").unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());

    let product = Product {
        id: 0, // Will be ignored
        name: req.name,
        cost_price: req.cost_price,
        sale_price: req.sale_price,
        current_stock: req.current_stock,
        weight_grams: req.weight_grams,
        status: req.status,
        production_date,
        expiration_date,
    };

    match ProductService::create_product(&pool, product).await {
        Ok(id) => Json(json!({"success": true, "message": "Produto criado com sucesso", "id": id})),
        Err(e) => Json(json!({"success": false, "message": format!("Erro ao criar produto: {:?}", e)})),
    }
}