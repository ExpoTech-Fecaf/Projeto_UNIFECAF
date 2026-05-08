//! Definição das rotas HTTP da API.
//!
//! Organiza todos os endpoints agrupados por domínio:
//! usuários, produtos, estoque, movimentações e relatórios.

// Função que organiza a estrutura das rotas principais da API
use axum::{routing::{get, post, put, delete}, Router, response::IntoResponse, Json};
use serde_json::json;
use crate::handlers::auth_handler;
use crate::handlers::product_handler;
use crate::handlers::stock_handler;
use sqlx::MySqlPool;

/// Handler de health check — retorna status da API.
async fn health_check() -> impl IntoResponse {
    Json(json!({"status": "ok", "message": "API is running"}))
}

/// Cria e retorna o router com todas as rotas da aplicação.
pub fn create_routes() -> Router<MySqlPool> {
    Router::new()
        // Rota de Health Check
        .route("/", get(health_check))

        // Rota usuário
        .route("/login", post(auth_handler::login))
        .route("/register", post(auth_handler::register))
        .route("/users/update/{id}", put(auth_handler::update_user))
        .route("/users/{id}", get(auth_handler::get_user))
        .route("/users/delete/{id}", delete(auth_handler::delete_user))
        .route("/users/promote", post(auth_handler::promote_user))

        // Rota para listar usuários (apenas para demonstração, deve ser protegida em produção)
        .route("/users", get(auth_handler::list_users))



        // Rotas para produtos
        .route("/products", get(product_handler::list_products))
        .route("/products/create", post(product_handler::create_product))
        .route("/products/update/{id}", put(product_handler::update_product))
        .route("/products/delete/{id}", delete(product_handler::delete_product))
        .route("/products/{id}", get(product_handler::get_product))

        // Rotas para movimentação de estoque
        .route("/products/stock/entry", post(stock_handler::stock_entry))
        .route("/products/stock/exit", post(stock_handler::stock_exit))
        .route("/products/stock/{name}", get(stock_handler::get_stock))

        // Histórico de movimentações
        .route("/movements", get(stock_handler::list_movements))
        .route("/movements/product/{product_id}", get(stock_handler::list_movements_by_product))

        // Relatórios
        .route("/reports/stock", get(stock_handler::stock_report))
        .route("/reports/critical", get(stock_handler::critical_stock_report))
        .route("/reports/alerts", get(stock_handler::consumption_alert))
        .route("/reports/low-stock", get(stock_handler::low_stock_warnings))
}
