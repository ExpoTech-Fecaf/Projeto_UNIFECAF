use axum::extract::{State, Json};
use sqlx::MySqlPool;
use serde_json::json;
use crate::services::stock_service;
use crate::repository::product_repository::ProductRepository;
use crate::repository::movement_repository::MovementRepository;
use crate::models::movement::Movement;

/// Payload de requisição para operações de estoque (entrada/saída).
#[derive(serde::Deserialize)]
pub struct StockRequest {
    /// Nome do produto
    pub product_name: String,
    /// Quantidade a movimentar
    pub quantity: i32,
    /// ID do usuário que realiza a operação
    pub user_id: i32,
    /// Observações opcionais
    #[serde(default)]
    pub notes: Option<String>,
}

/// Handler para entrada de estoque.
///
/// Adiciona a quantidade ao lote mais recente do produto e registra a movimentação.
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
                let movement = Movement {
                    id: None,
                    product_id: batch.id,
                    batch_id: Some(batch.id),
                    user_id: req.user_id,
                    movement_type: "entrada".to_string(),
                    quantity: req.quantity,
                    created_at: None,
                    notes: req.notes,
                };
                let _ = MovementRepository::create(&pool, &movement).await;

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

/// Handler para saída de estoque com lógica FIFO.
///
/// Consome lotes do mais antigo ao mais novo, registra a movimentação,
/// e retorna avisos de estoque baixo e consumo elevado (sem bloquear).
pub async fn stock_exit(
    State(pool): State<MySqlPool>,
    Json(req): Json<StockRequest>,
) -> Json<serde_json::Value> {
    let batches = match ProductRepository::find_batches_by_name(&pool, &req.product_name).await {
        Ok(b) => b,
        Err(_) => return Json(json!({"success": false, "message": "Produto não encontrado"})),
    };

    let product_id = match batches.first() {
        Some(b) => b.id,
        None => return Json(json!({"success": false, "message": "Nenhum lote encontrado"})),
    };

    match stock_service::withdraw_stock(&pool, &req.product_name, req.quantity).await {
        Ok(_) => {
            let movement = Movement {
                id: None,
                product_id,
                batch_id: Some(product_id),
                user_id: req.user_id,
                movement_type: "saida".to_string(),
                quantity: req.quantity,
                created_at: None,
                notes: req.notes,
            };
            let _ = MovementRepository::create(&pool, &movement).await;

            let remaining: i32 = ProductRepository::find_batches_by_name(&pool, &req.product_name)
                .await
                .map(|b| b.iter().map(|p| p.current_stock).sum())
                .unwrap_or(0);

            // Verificar aviso de estoque baixo (não bloqueia)
            let aviso = stock_service::verificar_estoque_baixo(&pool, &req.product_name).await;

            // Verificar alerta de consumo elevado para o dia (não bloqueia)
            let alerta_consumo = stock_service::verificar_consumo_elevado(&pool, &req.product_name, req.quantity).await;

            let mut response = json!({
                "success": true,
                "message": "Saída de estoque registrada",
                "product_name": req.product_name,
                "quantity_removed": req.quantity,
                "remaining_stock": remaining
            });

            if let Some(alerta) = aviso {
                response["aviso_estoque_baixo"] = json!({
                    "alerta": true,
                    "mensagem": alerta.mensagem,
                    "current_stock": alerta.current_stock,
                    "min_stock": alerta.min_stock
                });
            }

            if let Some(msg) = alerta_consumo {
                response["aviso_consumo_elevado"] = json!({
                    "alerta": true,
                    "mensagem": msg
                });
            }

            Json(response)
        }
        Err(e) => Json(json!({"success": false, "message": e})),
    }
}

/// Handler para consulta de estoque de um produto por nome.
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

/// Handler para listar todo o histórico de movimentações.
pub async fn list_movements(
    State(pool): State<MySqlPool>,
) -> Json<serde_json::Value> {
    match MovementRepository::list_all(&pool).await {
        Ok(movements) => Json(json!({
            "success": true,
            "data": movements
        })),
        Err(e) => Json(json!({"success": false, "message": format!("Erro: {}", e)})),
    }
}

/// Handler para listar movimentações de um produto específico.
pub async fn list_movements_by_product(
    State(pool): State<MySqlPool>,
    axum::extract::Path(product_id): axum::extract::Path<i32>,
) -> Json<serde_json::Value> {
    match MovementRepository::list_by_product(&pool, product_id).await {
        Ok(movements) => Json(json!({
            "success": true,
            "data": movements
        })),
        Err(e) => Json(json!({"success": false, "message": format!("Erro: {}", e)})),
    }
}

// ============================
// RELATÓRIOS
// ============================

/// Handler para relatório completo de estoque.
pub async fn stock_report(
    State(pool): State<MySqlPool>,
) -> Json<serde_json::Value> {
    match stock_service::gerar_relatorio_estoque(&pool).await {
        Ok(report) => Json(json!({
            "success": true,
            "data": report
        })),
        Err(e) => Json(json!({
            "success": false,
            "message": e
        })),
    }
}

/// Handler para relatório de produtos com estoque crítico (≤ 5 unidades).
pub async fn critical_stock_report(
    State(pool): State<MySqlPool>,
) -> Json<serde_json::Value> {
    match stock_service::produtos_estoque_critico(&pool).await {
        Ok(report) => Json(json!({
            "success": true,
            "data": report
        })),
        Err(e) => Json(json!({
            "success": false,
            "message": e
        })),
    }
}

/// Handler para alertas de consumo ajustados pelo dia da semana.
pub async fn consumption_alert(
    State(pool): State<MySqlPool>,
) -> Json<serde_json::Value> {
    match stock_service::alertas_consumo_dia(&pool).await {
        Ok(alertas) => {
            let em_alerta: Vec<_> = alertas.iter().filter(|a| a.alerta).collect();
            Json(json!({
                "success": true,
                "dia_semana": alertas.first().map(|a| a.dia_semana.clone()).unwrap_or_default(),
                "nivel_movimento": alertas.first().map(|a| a.nivel_movimento.clone()).unwrap_or_default(),
                "total_produtos": alertas.len(),
                "produtos_em_alerta": em_alerta.len(),
                "data": alertas
            }))
        }
        Err(e) => Json(json!({
            "success": false,
            "message": e
        })),
    }
}

/// Handler para listar todos os produtos com estoque abaixo do mínimo definido.
pub async fn low_stock_warnings(
    State(pool): State<MySqlPool>,
) -> Json<serde_json::Value> {
    match stock_service::listar_avisos_estoque_baixo(&pool).await {
        Ok(avisos) => Json(json!({
            "success": true,
            "total_avisos": avisos.len(),
            "data": avisos
        })),
        Err(e) => Json(json!({
            "success": false,
            "message": e
        })),
    }
}
