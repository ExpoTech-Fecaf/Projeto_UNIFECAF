use axum::{routing::{get, post}, Router};

// Função que organiza a estrutura das rotas principais da API
pub fn criar_rotas() -> Router {
    Router::new()
        // Rota para login de usuário
        .route("/login", post(login))
        
        // Rotas para produtos
        .route("/produtos/cadastro", post(cadastrar_p))
        .route("/produtos/listagem", get(listar_p))
        
        // Rotas para movimentação de estoque
        .route("/estoque/entrada", post(entrada_estoque))
        .route("/estoque/saida", post(saida_estoque))
}

// Definição das funções básicas (Estrutura inicial conforme solicitado)
async fn login() -> &'static str { "Estrutura de Login - Responsável: Samuel" }
async fn cadastrar_p() -> &'static str { "Estrutura de Cadastro de Produto - Responsável: Samuel" }
async fn listar_p() -> &'static str { "Estrutura de Listagem de Produtos - Responsável: Samuel" }
async fn entrada_estoque() -> &'static str { "Estrutura de Entrada de Estoque - Responsável: Samuel" }
async fn saida_estoque() -> &'static str { "Estrutura de Saída de Estoque - Responsável: Samuel" }