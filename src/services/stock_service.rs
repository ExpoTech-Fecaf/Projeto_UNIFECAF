use crate::models::product::Product;
use crate::repository::product_repository::ProductRepository;
use sqlx::{MySqlPool, Row};

/// Retira estoque de um produto seguindo a lógica FIFO (First In, First Out).
///
/// # Argumentos
/// * `pool` - Pool de conexões MySQL
/// * `product_name` - Nome do produto a ser retirado (ex: "Arroz", "Feijão")
/// * `quantity_to_remove` - Quantidade total a ser retirada do estoque
///
/// # Retorno
/// * `Ok(())` se a retirada for bem-sucedida
/// * `Err(String)` se houver erro (estoque insuficiente, produto não encontrado, etc)
///
/// # Lógica FIFO
/// 1. Busca todos os lotes ordenados por `entry_date` ASC
/// 2. Valida estoque total disponível
/// 3. Consome lotes do mais antigo ao mais novo até atingir a quantidade
/// 4. Persiste as alterações no banco
pub async fn withdraw_stock(pool: &MySqlPool, product_name: &str, mut quantity_to_remove: i32) -> Result<(), String> {
    if quantity_to_remove <= 0 {
        return Err("Quantidade a retirar deve ser maior que zero".to_string());
    }

    let mut batches = ProductRepository::find_batches_by_name(pool, product_name)
        .await
        .map_err(|_| format!("Erro ao buscar lotes para o produto: {}", product_name))?;

    if batches.is_empty() {
        return Err(format!("Nenhum lote encontrado para o produto: {}", product_name));
    }

    let total_stock: i32 = batches.iter().map(|b| b.current_stock).sum();
    if total_stock < quantity_to_remove {
        return Err(format!(
            "Estoque insuficiente. Disponível: {}, Solicitado: {}",
            total_stock, quantity_to_remove
        ));
    }

    let mut updates: Vec<(i32, i32)> = Vec::new();

    for batch in batches.iter_mut() {
        if quantity_to_remove <= 0 {
            break;
        }

        if batch.current_stock <= quantity_to_remove {
            quantity_to_remove -= batch.current_stock;
            updates.push((batch.id, 0));
            batch.current_stock = 0;
        } else {
            batch.current_stock -= quantity_to_remove;
            updates.push((batch.id, batch.current_stock));
            quantity_to_remove = 0;
        }
    }

    for (batch_id, new_quantity) in updates {
        ProductRepository::update_quantity(pool, batch_id, new_quantity)
            .await
            .map_err(|_| format!("Erro ao atualizar lote {}", batch_id))?;
    }

    Ok(())
}

/// Busca todos os lotes de um produto ordenados por data de entrada (FIFO).
pub async fn find_ordered_batches(pool: &MySqlPool, product_name: &str) -> Result<Vec<Product>, String> {
    ProductRepository::find_batches_by_name(pool, product_name)
        .await
        .map_err(|_| format!("Erro ao buscar lotes para o produto: {}", product_name))
}

/// Retira estoque de um produto identificado por ID, aplicando lógica FIFO.
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

    let mut batches = ProductRepository::find_batches_by_name(pool, &product_name)
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
}

// ============================
// RELATÓRIOS
// ============================

use crate::repository::product_repository::StockReport;

/// Gera relatório completo de estoque agrupado por produto.
pub async fn gerar_relatorio_estoque(
    pool: &MySqlPool,
) -> Result<Vec<StockReport>, String> {
    ProductRepository::stock_report(pool)
        .await
        .map_err(|_| "Erro ao gerar relatório".to_string())
}

/// Retorna produtos com estoque crítico (total ≤ 5 unidades).
pub async fn produtos_estoque_critico(
    pool: &MySqlPool,
) -> Result<Vec<StockReport>, String> {
    ProductRepository::critical_stock(pool)
        .await
        .map_err(|_| "Erro ao buscar estoque crítico".to_string())
}

// ============================
// AVISO DE ESTOQUE BAIXO
// ============================

use serde::Serialize;

/// Aviso de estoque baixo para um produto.
#[derive(Debug, Serialize)]
pub struct AvisoEstoqueBaixo {
    pub product_id: i32,
    pub product_name: String,
    pub current_stock: i32,
    pub min_stock: i32,
    pub mensagem: String,
}

/// Verifica se um produto específico está com estoque baixo.
///
/// Retorna `Some(AvisoEstoqueBaixo)` se estoque total ≤ `min_stock`.
pub async fn verificar_estoque_baixo(
    pool: &MySqlPool,
    product_name: &str,
) -> Option<AvisoEstoqueBaixo> {
    let batches = ProductRepository::find_batches_by_name(pool, product_name)
        .await
        .unwrap_or_default();

    if batches.is_empty() {
        return None;
    }

    let product_id = batches[0].id;
    let total_stock: i32 = batches.iter().map(|b| b.current_stock).sum();

    let min_stock = ProductRepository::get_min_stock(pool, product_id)
        .await
        .unwrap_or(0);

    if min_stock > 0 && total_stock <= min_stock {
        Some(AvisoEstoqueBaixo {
            product_id,
            product_name: product_name.to_string(),
            current_stock: total_stock,
            min_stock,
            mensagem: format!(
                "⚠ Aviso: estoque baixo. O produto \"{}\" está próximo de acabar. Quantidade atual: {} unidades.",
                product_name, total_stock
            ),
        })
    } else {
        None
    }
}

/// Lista todos os produtos com estoque abaixo do mínimo definido.
pub async fn listar_avisos_estoque_baixo(
    pool: &MySqlPool,
) -> Result<Vec<AvisoEstoqueBaixo>, String> {
    let products = ProductRepository::list(pool)
        .await
        .map_err(|_| "Erro ao buscar produtos".to_string())?;

    let mut avisos = Vec::new();

    for product in products {
        let min_stock = ProductRepository::get_min_stock(pool, product.id)
            .await
            .unwrap_or(0);

        if min_stock <= 0 {
            continue;
        }

        let batches = ProductRepository::find_batches_by_name(pool, &product.name)
            .await
            .unwrap_or_default();

        let total_stock: i32 = batches.iter().map(|b| b.current_stock).sum();

        if total_stock <= min_stock {
            avisos.push(AvisoEstoqueBaixo {
                product_id: product.id,
                product_name: product.name.clone(),
                current_stock: total_stock,
                min_stock,
                mensagem: format!(
                    "⚠ Aviso: estoque baixo. O produto \"{}\" está próximo de acabar. Quantidade atual: {} unidades.",
                    product.name, total_stock
                ),
            });
        }
    }

    Ok(avisos)
}

// ============================
// ALERTA DE CONSUMO ELEVADO NA SAÍDA
// ============================

use crate::models::consumo::DiaSemana;
use chrono::Datelike;

/// Verifica se a quantidade sendo retirada excede o limite recomendado para o dia.
///
/// Calcula: min_stock × multiplicador do dia da semana.
/// Se a quantidade solicitada for maior que o limite, retorna um aviso (não bloqueia).
pub async fn verificar_consumo_elevado(
    pool: &MySqlPool,
    product_name: &str,
    quantidade_retirada: i32,
) -> Option<String> {
    let batches = ProductRepository::find_batches_by_name(pool, product_name)
        .await
        .unwrap_or_default();

    if batches.is_empty() {
        return None;
    }

    let product_id = batches[0].id;

    let min_stock = ProductRepository::get_min_stock(pool, product_id)
        .await
        .unwrap_or(0);

    if min_stock <= 0 {
        return None;
    }

    let hoje = chrono::Local::now().naive_local().date();
    let dia = DiaSemana::from_chrono(hoje.weekday());
    let nivel = dia.nivel();
    let multiplicador = nivel.multiplicador();
    let limite_recomendado = (min_stock as f64 * multiplicador).ceil() as i32;

    if quantidade_retirada > limite_recomendado {
        Some(format!(
            "⚠ Alerta: quantidade elevada para o dia. Limite recomendado para {:?} ({}): {} unidades. Solicitado: {} unidades.",
            dia, nivel.descricao(), limite_recomendado, quantidade_retirada
        ))
    } else {
        None
    }
}

// ============================
// ALERTA DE CONSUMO POR DIA DA SEMANA
// ============================

#[derive(Debug, Serialize)]
pub struct AlertaConsumo {
    pub product_id: i32,
    pub product_name: String,
    pub current_stock: i32,
    pub min_stock_original: i32,
    pub min_stock_ajustado: i32,
    pub dia_semana: String,
    pub nivel_movimento: String,
    pub multiplicador: f64,
    pub alerta: bool,
    pub mensagem: String,
}

/// Gera alertas de consumo ajustados pelo dia da semana atual.
///
/// O sistema calcula o `min_stock` ajustado multiplicando o valor base
/// pelo fator do dia da semana:
/// - Segunda (Baixo): min_stock × 0.5
/// - Terça/Quarta (Médio): min_stock × 1.0
/// - Quinta/Sexta (Alto): min_stock × 1.3
/// - Sábado/Domingo (Muito Alto): min_stock × 1.6
pub async fn alertas_consumo_dia(
    pool: &MySqlPool,
) -> Result<Vec<AlertaConsumo>, String> {
    let hoje = chrono::Local::now().naive_local().date();
    let dia = DiaSemana::from_chrono(hoje.weekday());
    let nivel = dia.nivel();
    let multiplicador = nivel.multiplicador();

    let products = ProductRepository::list(pool)
        .await
        .map_err(|_| "Erro ao buscar produtos".to_string())?;

    let mut alertas = Vec::new();

    for product in products {
        let batches = ProductRepository::find_batches_by_name(pool, &product.name)
            .await
            .unwrap_or_default();

        let total_stock: i32 = batches.iter().map(|b| b.current_stock).sum();

        let min_stock_original = ProductRepository::get_min_stock(pool, product.id)
            .await
            .unwrap_or(0);

        let min_stock_ajustado = (min_stock_original as f64 * multiplicador).ceil() as i32;
        let em_alerta = total_stock <= min_stock_ajustado;

        let mensagem = if em_alerta {
            format!(
                "⚠️ ALERTA: Estoque de '{}' ({} un.) está abaixo do mínimo ajustado ({} un.) para {}",
                product.name, total_stock, min_stock_ajustado, nivel.descricao()
            )
        } else {
            format!(
                "✅ '{}' com estoque adequado ({} un.) para o dia",
                product.name, total_stock
            )
        };

        alertas.push(AlertaConsumo {
            product_id: product.id,
            product_name: product.name,
            current_stock: total_stock,
            min_stock_original,
            min_stock_ajustado,
            dia_semana: format!("{:?}", dia),
            nivel_movimento: nivel.descricao().to_string(),
            multiplicador,
            alerta: em_alerta,
            mensagem,
        });
    }

    Ok(alertas)
}
