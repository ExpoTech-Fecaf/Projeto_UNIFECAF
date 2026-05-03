use std::env;
use dotenv::dotenv;
// Importa o gerenciador de conexões MySQL
use sqlx::MySqlPool;   // importa o gerenciador de conexões MySQL

pub async fn connect_db() -> Result<MySqlPool, Box<dyn std::error::Error>> {
    dotenv().ok();  // inicia o dotenv para ler o arquivo de configuração

    // Lê a URL do banco de dados da variável de ambiente
    let database_url = env::var("DATABASE_URL")?;

    // Conecta ao MySQL de forma assíncrona e retorna a 'pool' de conexões
    let pool = MySqlPool::connect(&database_url).await?;
    Ok(pool)
}