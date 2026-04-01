use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Definindo os caminhos (Rotas)
    let app = Router::new()
        .route("/", get(|| async { "Sistema de Estoque Rodando! - Samuel" }))
        .route("/login", post(login))
        .route("/produtos", post(cadastrar_p))
        .route("/produtos", get(listar_p))
        .route("/estoque/entrada", post(entrada))
        .route("/estoque/saida", post(saida));

    // Endereço do servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 O SISTEMA ESTÁ ON EM: http://{}", addr);

    // Liga o motor
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Espaços reservados para o Dan e o Icaro
async fn login() -> &'static str { "Aguardando lógica do Icaro" }
async fn cadastrar_p() -> &'static str { "Aguardando lógica do Dan" }
async fn listar_p() -> &'static str { "Aguardando lógica do Dan" }
async fn entrada() -> &'static str { "Aguardando lógica do Dan" }
async fn saida() -> &'static str { "Aguardando lógica do Dan" } 