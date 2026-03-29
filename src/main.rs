mod config; //declara o modulo 'config' para o Rust encontrar a pasta

#[tokio::main]  //define que o 'main' pode rodar tarefas assíncronas
async fn main() {
    println!("Iniciando conexão");  //log informativo mo terminal

    //chama a funçaõ de conexão e aguarda (.await) o banco responder
    let pool = config::database::conectar_db().await;

    //mensagem para a conexão bem-sucedida
    println!("Conexão com banco realizada com sucesso!");
}
