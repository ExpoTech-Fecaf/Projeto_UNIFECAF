use crate::models::product::Product;
use crate::repository::product_repository::ProductRepository;
use crate::models::dia_semana::{DiaSemana, NivelMovimento};
use sqlx::{MySqlPool, Row};
use diesel::prelude::*;
use crate::models::movement::Movement;
use crate::schema::movements; 
use crate::repository::product_repository::ProductRepository;

/// Retira estoque de um produto seguindo a lógica FIFO (First In, First Out).
/// 
/// # Argumentos
/// * `product_name` - Nome do produto a ser retirado (ex: "Arroz", "Feijão")
/// * `quantity_to_remove` - Quantidade total a ser retirada do estoque
/// 
/// # Retorno
/// * `Ok(())` se a retirada for bem-sucedida
/// * `Err(String)` se houver erro (estoque insuficiente, produto não encontrado, etc)
/// 
/// # Lógica FIFO
/// Busca todos os lotes (linhas de produto) com esse nome, ordenados por entry_date (ASC).
/// Consome primeiro o lote mais antigo até alcançar a quantidade solicitada.
pub async fn withdraw_stock(pool: &MySqlPool, product_name: &str, mut quantity_to_remove: i32) -> Result<(), String> {
    if quantity_to_remove <= 0 {
        return Err("Quantidade a retirar deve ser maior que zero".to_string());
    }

    // 1. Busca todos os lotes (produtos com o mesmo nome), ordenados por entry_date ASC (FIFO)
    let mut batches = ProductRepository::find_batches_by_name(pool, product_name)
        .await
        .map_err(|_| format!("Erro ao buscar lotes para o produto: {}", product_name))?;

    if batches.is_empty() {
        return Err(format!("Nenhum lote encontrado para o produto: {}", product_name));
    }

    // 2. Verifica se a soma total de todos os lotes é suficiente antes de começar
    let total_stock: i32 = batches.iter().map(|b| b.current_stock).sum();
    if total_stock < quantity_to_remove {
        return Err(format!(
            "Estoque insuficiente. Disponível: {}, Solicitado: {}",
            total_stock, quantity_to_remove
        ));
    }

    // 3. Itera pelos lotes na ordem FIFO (do mais antigo para o mais novo)
    let mut updates: Vec<(i32, i32)> = Vec::new(); // (batch_id, novo_estoque)

    for batch in batches.iter_mut() {
        if quantity_to_remove <= 0 {
            break;
        }

        if batch.current_stock <= quantity_to_remove {
            // Consome todo o lote
            quantity_to_remove -= batch.current_stock;
            updates.push((batch.id, 0)); // Ajusta o lote para zero
            batch.current_stock = 0;
        } else {
            // Consome apenas o necessário do lote atual
            batch.current_stock -= quantity_to_remove;
            updates.push((batch.id, batch.current_stock));
            quantity_to_remove = 0;
        }
    }

    // 4. Persiste as alterações no banco de dados
    for (batch_id, new_quantity) in updates {
        ProductRepository::update_quantity(pool, batch_id, new_quantity)
            .await
            .map_err(|_| format!("Erro ao atualizar lote {}", batch_id))?;
    }

    // 5. Registro de movimentação (Tarefa 2)
    // TODO: Tarefa 2 - Registrar movimento chamando a função de histórico
    // Por exemplo: register_movement(product_name, initial_quantity_to_remove).await?;

    Ok(())
}

/// Busca todos os lotes (produtos) de um tipo específico, ordenados por entry_date (FIFO).
pub async fn find_ordered_batches(pool: &MySqlPool, product_name: &str) -> Result<Vec<Product>, String> {
    ProductRepository::find_batches_by_name(pool, product_name)
        .await
        .map_err(|_| format!("Erro ao buscar lotes para o produto: {}", product_name))
}

pub async fn retirar_estoque(pool: &MySqlPool, produto_id: i32, mut quantidade: i32) -> Result<(), String> {
    if quantidade <= 0 {
        return Err("Quantidade a retirar deve ser maior que zero".to_string());
    }

    let row = sqlx::query("SELECT nome FROM produto WHERE id = ?")
        .bind(produto_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| "Erro ao buscar o produto".to_string())?;

    let product_name: String = row
        .ok_or_else(|| format!("Produto não encontrado: {}", produto_id))?
        .get("nome");

    let mut batches = ProductRepository::buscar_estoque_total(pool, &product_name)
        .await
        .map_err(|_| format!("Erro ao buscar lotes para o produto: {}", product_name))?;

    if batches.is_empty() {
        return Err(format!("Nenhum lote encontrado para o produto: {}", product_name));
    }

    let total_stock: i32 = batches.iter().map(|b| b.current_stock).sum();
    if total_stock < quantidade {
        return Err(format!(
            "Estoque insuficiente. Disponível: {}, Solicitado: {}",
            total_stock, quantidade
        ));
    }

    let mut updates: Vec<(i32, i32)> = Vec::new();
    for batch in batches.iter_mut() {
        if quantidade <= 0 {
            break;
        }

        if batch.current_stock <= quantidade {
            quantidade -= batch.current_stock;
            updates.push((batch.id, 0));
        } else {
            batch.current_stock -= quantidade;
            updates.push((batch.id, batch.current_stock));
            quantidade = 0;
        }
    }

    for (batch_id, new_quantity) in updates {
        ProductRepository::update_quantity(pool, batch_id, new_quantity)
            .await
            .map_err(|_| format!("Erro ao atualizar lote {}", batch_id))?;
    }

    Ok(())

pub fn calcular_limite_diario(limite_base: f64, dia_semana: DiaSemana) -> f64 {
    let nivel = dia_semana.nivel();
    limite_base * nivel.multiplicador()
}


pub fn registrar_movimentacao(
    conn: &mut PgConnection,
    movement: Movement,
) -> Result<(), diesel::result::Error> {
    diesel::insert_into(movements::table)
        .values(&movement)
        .execute(conn)?;
    Ok(())
}

pub fn checar_estoque_baixo(
    product_repo: &impl ProductRepository,
    produto_id: i32,
    quantidade_minima: i32,
) -> Option<String> {
    let estoque_total = product_repo.find_batches_by_name(produto_id).ok()?;
    if estoque_total <= quantidade_minima {
        Some(format!(
            "Aviso: Estoque baixo para o produto {}! Quantidade atual: {} (mínimo: {})",
            produto_id, estoque_total, quantidade_minima
        ))
    } else {
        None
    }
}

pub fn registrar_saida(
    conn: &mut PgConnection,
    produto_id: i32,
    quantidade: i32,
    usuario_id: i32,
    product_repo: &impl ProductRepository,
    quantidade_minima: i32,
) -> Result<(), String> {
    // 1. Registra a movimentação
    let movement = Movement {
        tipo_movimento: TipoMovimento::Saida,
        produto_id,
        quantidade,
        usuario_id,
        data_hora: chrono::Local::now().naive_local(),
    };
    registrar_movimentacao(conn, movement).map_err(|e| e.to_string())?;

    // 2. Checa estoque baixo
    if let Some(aviso) = checar_estoque_baixo(product_repo, produto_id, quantidade_minima) {
        println!("{}", aviso);
    }

    Ok(())
}

}