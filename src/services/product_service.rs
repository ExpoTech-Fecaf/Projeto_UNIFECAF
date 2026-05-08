use crate::repository::product_repository::ProductRepository;
use crate::models::product::Product;
use sqlx::MySqlPool;

/// Serviço de negócio para operações CRUD de produtos.
///
/// Camada intermediária entre handlers e repositório.
pub struct ProductService;

impl ProductService {
    /// Cria um novo produto e retorna o ID gerado.
    pub async fn create_product(pool: &MySqlPool, product: Product) -> Result<i32, sqlx::Error> {
        ProductRepository::create(pool, &product).await
    }

    /// Lista todos os produtos cadastrados.
    pub async fn list_products(pool: &MySqlPool) -> Result<Vec<Product>, sqlx::Error> {
        ProductRepository::list(pool).await
    }

    /// Atualiza um produto existente.
    pub async fn update_product(pool: &MySqlPool, product: Product) -> Result<(), sqlx::Error> {
        ProductRepository::update(pool, &product).await
    }

    /// Remove um produto pelo ID.
    pub async fn delete_product(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        ProductRepository::delete(pool, id).await
    }
}