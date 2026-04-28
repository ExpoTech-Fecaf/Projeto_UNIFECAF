use sqlx::PgPool;
use crate::models::Produto;

pub struct ProdutoRepository;

impl ProdutoRepository {
    pub async fn criar(pool: &PgPool, produto: &Produto) -> Result<i32, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            INSERT INTO produto (nome, valorcusto, valorvenda, estoqueatual, pesogramas, status, dataproducao, datavalidade)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#,
            produto.nome,
            produto.valor_custo,
            produto.valor_venda,
            produto.estoque_atual,
            produto.peso_gramas,
            produto.status,
            produto.data_producao,
            produto.data_validade
        )
        .fetch_one(pool)
        .await?;
        Ok(row.id)
    }

    pub async fn listar(pool: &PgPool) -> Result<Vec<Produto>, sqlx::Error> {
        let produtos = sqlx::query_as!(Produto,
            r#"
            SELECT id, nome as "nome!", valorcusto as "valor_custo!", valorvenda as "valor_venda!",
                   estoqueatual as "estoque_atual!", pesogramas as "peso_gramas!", status as "status!",
                   dataproducao as "data_producao!", datavalidade as "data_validade!"
            FROM produto
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(produtos)
    }

    pub async fn atualizar(pool: &PgPool, produto: &Produto) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE produto
            SET nome = $1, valorcusto = $2, valorvenda = $3, estoqueatual = $4,
                pesogramas = $5, status = $6, dataproducao = $7, datavalidade = $8
            WHERE id = $9
            "#,
            produto.nome,
            produto.valor_custo,
            produto.valor_venda,
            produto.estoque_atual,
            produto.peso_gramas,
            produto.status,
            produto.data_producao,
            produto.data_validade,
            produto.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn deletar(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM produto WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
