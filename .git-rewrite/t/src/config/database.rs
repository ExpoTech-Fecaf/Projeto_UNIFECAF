use std::env;
use dotenv::dotenv;
// Importa o gerenciador de conexões MySQL
use sqlx::MySqlPool;   //importa o gerenciador de conexões MySQL

pub async fn conectar_db() -> MySqlPool {
    dotenv().ok();  //inicia o dotenv para ler p arquivo de configuração

    // Lê a URL do banco de dados da variável de ambiente
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definida no .env");

    // Conecta ao MySQL de forma assíncrona e retorna a 'pool' de conexões
    MySqlPool::connect(&database_url)
        .await
        .expect("Erro ao conectar no banco")
}