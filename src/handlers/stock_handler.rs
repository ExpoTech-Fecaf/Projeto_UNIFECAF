use axum::extract::{State, Json};
use sqlx::MySqlPool;
use serde_json::json;
use crate::services::stock_service;
use crate::repository::product_repository::ProductRepository;

#[derive(serde::Deserialize)]
pub struct StockRequest {
    pub product_name: String,
    pub quantity: i32,
}

// Entrada de estoque
pub async fn stock_entry(
    State(pool): State<MySqlPool>,
    Json(req): Json<StockRequest>,
) -> Json<serde_json::Value> {
    if req.quantity <= 0 {
        return Json(json!({"success": false, "message": "Quantidade deve ser maior que zero"}));
    }

    let batches = match ProductRepository::find_batches_by_name(&pool, &req.product_name).await {
        Ok(b) => b,
        Err(_) => return Json(json!({"success": false, "message": "Produto não encontrado"})),
    };

    if let Some(batch) = batches.last() {
        let new_stock = batch.current_stock + req.quantity;
        match ProductRepository::update_quantity(&pool, batch.id, new_stock).await {
            Ok(_) => {
                let total: i32 = batches.iter().map(|b| b.current_stock).sum::<i32>() + req.quantity;
                Json(json!({
                    "success": true,
                    "message": "Entrada de estoque registrada",
                    "product_name": req.product_name,
                    "quantity_added": req.quantity,
                    "total_stock": total
                }))
            }
            Err(e) => Json(json!({"success": false, "message": format!("Erro: {}", e)})),
        }
    } else {
        Json(json!({"success": false, "message": format!("Nenhum lote encontrado para: {}", req.product_name)}))
    }
}

// Saída de estoque (FIFO)
pub async fn stock_exit(
    State(pool): State<MySqlPool>,
    Json(req): Json<StockRequest>,
) -> Json<serde_json::Value> {
    match stock_service::withdraw_stock(&pool, &req.product_name, req.quantity).await {
        Ok(_) => {
            let remaining: i32 = ProductRepository::find_batches_by_name(&pool, &req.product_name)
                .await
                .map(|b| b.iter().map(|p| p.current_stock).sum())
                .unwrap_or(0);

            Json(json!({
                "success": true,
                "message": "Saída de estoque registrada",
                "product_name": req.product_name,
                "quantity_removed": req.quantity,
                "remaining_stock": remaining
            }))
        }
        Err(e) => Json(json!({"success": false, "message": e})),
    }
}

// Consultar estoque de um produto por nome
pub async fn get_stock(
    State(pool): State<MySqlPool>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    match ProductRepository::find_batches_by_name(&pool, &name).await {
        Ok(batches) => {
            let total: i32 = batches.iter().map(|b| b.current_stock).sum();
            let details: Vec<serde_json::Value> = batches.iter().map(|b| {
                json!({
                    "batch_id": b.id,
                    "current_stock": b.current_stock,
                    "entry_date": b.entry_date.to_string()
                })
            }).collect();

            Json(json!({
                "product_name": name,
                "total_stock": total,
                "batches": details
            }))
        }
        Err(_) => Json(json!({"success": false, "message": "Produto não encontrado"})),
    }
}
