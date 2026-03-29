use sqlx::MySqlPool;    //importa o gerenciador de conexões MySQL
use std::env;   //permite acessar variáveis de ambiente do sistema
use dotenv::dotenv; //carrega o arquivo .env para o Projeto

pub async fn conectar_db() -> MySqlPool {
    dotenv().ok();  //inicia o dotenv para ler p arquivo de configuraçaõ

    //busca a URL de conexão e para o programa se ela não existir
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL não definida");

    //conecta ao MySQL de forma assíncrona e retorna a 'pool' de conexões
    MySqlPool::connect(&database_url)
        .await
        .expect("Erro ao conectar no banco")
}