//! Configuração de conexão com o banco de dados MySQL.
//!
//! Utiliza variável de ambiente `DATABASE_URL` definida no arquivo `.env`.

use std::env;
use dotenv::dotenv;
use sqlx::MySqlPool;

/// Estabelece conexão com o banco de dados MySQL.
///
/// Lê a `DATABASE_URL` do arquivo `.env` e retorna um pool de conexões
/// gerenciado pelo SQLx para uso assíncrono.
///
/// # Erros
/// Retorna erro se a variável `DATABASE_URL` não estiver definida
/// ou se a conexão com o banco falhar.
pub async fn connect_db() -> Result<MySqlPool, Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    let pool = MySqlPool::connect(&database_url).await?;
    Ok(pool)
}