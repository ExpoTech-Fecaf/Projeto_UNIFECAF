use crate::repository::product_repository::ProductRepository;
use crate::models::product::Product;
use sqlx::MySqlPool;

pub struct ProductService;

impl ProductService {
    pub async fn create_product(pool: &MySqlPool, product: Product) -> Result<i32, sqlx::Error> {
        ProductRepository::create(pool, &product).await
    }

    pub async fn list_products(pool: &MySqlPool) -> Result<Vec<Product>, sqlx::Error> {
        ProductRepository::list(pool).await
    }

    pub async fn update_product(pool: &MySqlPool, product: Product) -> Result<(), sqlx::Error> {
        ProductRepository::update(pool, &product).await
    }

    pub async fn delete_product(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        ProductRepository::delete(pool, id).await
    }
}