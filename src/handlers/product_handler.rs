use axum::extract::{State, Path, Json};
use axum::http::StatusCode;
use sqlx::MySqlPool;
use serde_json::json;
use crate::models::product::Product;
use crate::services::product_service::ProductService;
use crate::validators::product_validator::ProductValidator;

/// Payload de requisição para criação de produto.
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

/// Handler para criação de produto.
///
/// Valida nome único, status e datas antes de persistir.
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
        id: 0,
        name: req.name.trim().to_string(),
        cost_price: req.cost_price,
        sale_price: req.sale_price,
        current_stock: req.current_stock,
        weight_grams: req.weight_grams,
        status,
        production_date,
        expiration_date,
        entry_date: chrono::Local::now().naive_local().date(),
    };

    match ProductService::create_product(&pool, product).await {
        Ok(id) => Json(json!({"success": true, "message": "Produto criado com sucesso", "id": id})),
        Err(e) => Json(json!({"success": false, "message": format!("Erro ao criar produto: {:?}", e)})),
    }
}

/// Handler para listagem de todos os produtos.
pub async fn list_products(State(pool): State<MySqlPool>) -> Json<Vec<Product>> {
    match ProductService::list_products(&pool).await {
        Ok(products) => Json(products),
        Err(_) => Json(vec![]),
    }
}

/// Handler para busca de produto por ID.
pub async fn get_product(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<Product>, (StatusCode, Json<serde_json::Value>)> {
    match ProductService::list_products(&pool).await {
        Ok(products) => {
            if let Some(product) = products.into_iter().find(|p| p.id == id) {
                Ok(Json(product))
            } else {
                Err((StatusCode::NOT_FOUND, Json(json!({"message": "Produto não encontrado"}))))
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": e.to_string()}))))
    }
}

/// Handler para atualização de produto.
///
/// Valida nome único (excluindo o próprio) e status antes de atualizar.
pub async fn update_product(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
    Json(mut payload): Json<Product>,
) -> Json<serde_json::Value> {
    payload.id = id;

    let existing_products = ProductService::list_products(&pool).await.unwrap_or_default();
    if existing_products.iter().any(|p| p.name.to_lowercase() == payload.name.to_lowercase() && p.id != id) {
        return Json(json!({
            "success": false, 
            "message": "Já existe outro produto com este nome"
        }));
    }

    if let Err(e) = ProductValidator::validate_status(payload.status) {
        return Json(json!({"success": false, "message": e.message}));
    }

    match ProductService::update_product(&pool, payload).await {
        Ok(_) => Json(json!({
            "success": true,
            "message": "Produto atualizado com sucesso"
        })),
        Err(e) => Json(json!({
            "success": false,
            "message": format!("Erro ao atualizar no banco: {}", e)
        })),
    }
}

/// Handler para remoção de produto por ID.
pub async fn delete_product(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> StatusCode {
    match ProductService::delete_product(&pool, id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
