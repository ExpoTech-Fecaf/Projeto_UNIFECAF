use crate::repository::UsuarioRepository;
use crate::models::Usuario;
use sqlx::PgPool;

pub struct UsuarioService;

impl UsuarioService {
    pub async fn criar_usuario(pool: &PgPool, usuario: Usuario) -> Result<i32, sqlx::Error> {
        UsuarioRepository::criar(pool, &usuario).await
    }

    pub async fn listar_usuarios(pool: &PgPool) -> Result<Vec<Usuario>, sqlx::Error> {
        UsuarioRepository::listar(pool).await
    }

    pub async fn atualizar_usuario(pool: &PgPool, usuario: Usuario) -> Result<(), sqlx::Error> {
        UsuarioRepository::atualizar(pool, &usuario).await
    }

    pub async fn deletar_usuario(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
        UsuarioRepository::deletar(pool, id).await
    }
}