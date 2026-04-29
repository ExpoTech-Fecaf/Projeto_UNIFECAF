use sqlx::PgPool;
use crate::models::Usuario;

pub struct UsuarioRepository;

impl UsuarioRepository {
    pub async fn criar(pool: &PgPool, usuario: &Usuario) -> Result<i32, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            INSERT INTO usuario (nome, sobrenome, cpf, datanascimento, user, senha, fkidcargo)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#,
            usuario.nome,
            usuario.sobrenome,
            usuario.cpf,
            usuario.data_nascimento,
            usuario.user,
            usuario.senha,
            usuario.fkidcargo
        )
        .fetch_one(pool)
        .await?;
        Ok(row.id)
    }

    pub async fn listar(pool: &PgPool) -> Result<Vec<Usuario>, sqlx::Error> {
        let usuarios = sqlx::query_as!(Usuario,
            r#"
            SELECT id, nome, sobrenome, cpf, datanascimento as "data_nascimento!",
                   user, senha, fkidcargo
            FROM usuario
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(usuarios)
    }


    pub async fn atualizar(pool: &PgPool, usuario: &Usuario) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE usuario
            SET nome = $1, sobrenome = $2, cpf = $3, datanascimento = $4,
                user = $5, senha = $6, fkidcargo = $7
            WHERE id = $8
            "#,
            usuario.nome,
            usuario.sobrenome,
            usuario.cpf,
            usuario.data_nascimento,
            usuario.user,
            usuario.senha,
            usuario.fkidcargo,
            usuario.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn deletar(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM usuario WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}