mod config;

#[tokio::main]
async fn main() {
    println!("Iniciando conexão");

    let pool = config::database::conectar_db().await;

    println!("Conexão com banco realizada com sucesso!");

    // Adicionando uma rota para testar o login
    use axum::{routing::post, Router};
    use gerenciamento_de_estoque::handlers::auth_handler::login;
    use axum::serve;
    use tokio::net::TcpListener;

    let app = Router::new()
        .route("/test_login", post(login))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    serve(listener, app).await.unwrap();
}