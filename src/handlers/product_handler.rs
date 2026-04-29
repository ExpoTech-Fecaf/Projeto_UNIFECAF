use axum::extract::{State, Json};
use sqlx::MySqlPool;
use serde_json::json;
use crate::models::product::Product;
use crate::services::product_service::ProductService;
use crate::validators::product_validator::ProductValidator;

#[derive(serde::Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub cost_price: f64,
    pub sale_price: f64,
    pub current_stock: i32,
    pub weight_grams: i32,
    #[serde(default)]
    pub status: Option<i16>,
    pub production_date: String,
    pub expiration_date: String,
}

pub async fn create_product(
    State(pool): State<MySqlPool>,
    Json(req): Json<CreateProductRequest>,
) -> Json<serde_json::Value> {
    if let Err(e) = ProductValidator::validate_name_unique(&pool, &req.name).await {
        return Json(json!({"success": false, "message": e.message, "field": e.field}));
    }

    let status = req.status.unwrap_or(1);
    if let Err(e) = ProductValidator::validate_status(status) {
        return Json(json!({"success": false, "message": e.message, "field": e.field}));
    }

    let production_date = match ProductValidator::validate_and_parse_date(&req.production_date, "production_date") {
        Ok(date) => date,
        Err(e) => return Json(json!({"success": false, "message": e.message, "field": e.field})),
    };

    let expiration_date = match ProductValidator::validate_and_parse_date(&req.expiration_date, "expiration_date") {
        Ok(date) => date,
        Err(e) => return Json(json!({"success": false, "message": e.message, "field": e.field})),
    };

    let product = Product {
        id: 0, // Will be ignored
        name: req.name.trim().to_string(),
        cost_price: req.cost_price,
        sale_price: req.sale_price,
        current_stock: req.current_stock,
        weight_grams: req.weight_grams,
        status,
        production_date,
        expiration_date,
    };

    match ProductService::create_product(&pool, product).await {
        Ok(id) => Json(json!({"success": true, "message": "Produto criado com sucesso", "id": id})),
        Err(e) => Json(json!({"success": false, "message": format!("Erro ao criar produto: {:?}", e)})),
    }
}