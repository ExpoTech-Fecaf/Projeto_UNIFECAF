use axum::serve;
use tokio::net::TcpListener;
use log::{info, error};
use gerenciamento_de_estoque::config;
use gerenciamento_de_estoque::routes;

#[tokio::main]
async fn main() {
    // Inicializar o logger
    env_logger::init();

    info!("🚀 Iniciando aplicação de Gerenciamento de Estoque");
    info!("📡 Conectando ao banco de dados...");

    // Conectar ao banco de dados
    match config::database::connect_db().await {
        Ok(pool) => {
            info!("✅ Conexão com banco de dados realizada com sucesso!");

            // Criar as rotas
            let app = routes::route::create_routes()
                .with_state(pool.clone());

            // Definir o endereço e porta
            let addr = "0.0.0.0:3001";
            let listener = match TcpListener::bind(addr).await {
                Ok(listener) => {
                    info!("🌐 Servidor iniciando em http://{}", addr);
                    listener
                }
                Err(e) => {
                    error!("❌ Erro ao fazer bind do servidor: {}", e);
                    return;
                }
            };

            // Iniciar o servidor
            match serve(listener, app).await {
                Ok(_) => info!("✅ Servidor finalizado"),
                Err(e) => error!("❌ Erro ao executar servidor: {}", e),
            }
        }
        Err(e) => {
            error!("❌ Erro ao conectar no banco de dados: {}", e);
            error!("Verifique se:");
            error!("1. O arquivo .env está configurado com DATABASE_URL");
            error!("2. O banco de dados está rodando");
            error!("3. As credenciais estão corretas");
        }
    }
}