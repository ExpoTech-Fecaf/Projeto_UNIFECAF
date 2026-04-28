use crate::repository::ProdutoRepository;
use crate::models::Produto;
use sqlx::PgPool;

pub struct ProdutoService;

impl ProdutoService {
    pub async fn criar_produto(pool: &PgPool, produto: Produto) -> Result<i32, sqlx::Error> {
        ProdutoRepository::criar(pool, &produto).await
    }

    pub async fn listar_produtos(pool: &PgPool) -> Result<Vec<Produto>, sqlx::Error> {
        ProdutoRepository::listar(pool).await
    }

    pub async fn atualizar_produto(pool: &PgPool, produto: Produto) -> Result<(), sqlx::Error> {
        ProdutoRepository::atualizar(pool, &produto).await
    }

    pub async fn deletar_produto(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
        ProdutoRepository::deletar(pool, id).await
    }
}