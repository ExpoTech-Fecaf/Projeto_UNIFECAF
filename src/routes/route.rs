// Função que organiza a estrutura das rotas principais da API
use axum::{routing::{get, post}, Router, response::IntoResponse, Json};
use serde_json::json;
use crate::handlers::auth_handler;
use crate::handlers::product_handler;
use sqlx::MySqlPool;

// Handlers simples para demonstração
async fn health_check() -> impl IntoResponse {
    Json(json!({"status": "ok", "message": "API is running"}))
}

async fn product_list() -> impl IntoResponse {
    Json(json!({"message": "Produtos listados com sucesso"}))
}

async fn stock_entry() -> impl IntoResponse {
    Json(json!({"message": "Entrada de estoque registrada"}))
}

async fn stock_exit() -> impl IntoResponse {
    Json(json!({"message": "Saída de estoque registrada"}))
}

// Função que organiza a estrutura das rotas principais da API
pub fn create_routes() -> Router<MySqlPool> {
    Router::new()
        // Rota de Health Check
        .route("/", get(health_check))

        // Rota para login de usuário
        .route("/login", post(auth_handler::login))

        // Rota para registro de usuário
        .route("/register", post(auth_handler::register))

        // Rotas para produtos
        .route("/produtos", get(product_list))
        .route("/produtos/criar", post(product_handler::create_product))

        // Rotas para movimentação de estoque
        .route("/estoque/entrada", post(stock_entry))
        .route("/estoque/saida", post(stock_exit))
}
