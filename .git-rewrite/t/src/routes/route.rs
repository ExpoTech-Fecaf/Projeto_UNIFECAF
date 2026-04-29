// Função que organiza a estrutura das rotas principais da API
use axum::{routing::{get, post}, Router};

use product_handler::{cadastrar_p, listar_p};

use stock_handler::{entrada_estoque, saida_estoque};

// Função que organiza a estrutura das rotas principais da API
pub fn criar_rotas() -> Router {
    Router::new()
        // Rota para login de usuário
        .route("/login", post(auth_handler::login))

        // Rotas para produtos
        .route("/produtos/cadastro", post(cadastrar_p))
        .route("/produtos/listagem", get(listar_p))

        // Rotas para movimentação de estoque
        .route("/estoque/entrada", post(entrada_estoque))
        .route("/estoque/saida", post(saida_estoque))
}

// Definição das funções básicas (Estrutura inicial conforme solicitado)
async fn login() -> &'static str { "Estrutura de Login - Responsável: Samuel" }
