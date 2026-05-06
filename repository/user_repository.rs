use sqlx::MySqlPool;
use sqlx::Row;
use crate::models::user::User;

pub struct UserRepository;

impl UserRepository {
    pub async fn create(pool: &MySqlPool, user: &User) -> Result<i32, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO usuario (nome, sobrenome, cpf, datanascimento, user, senha, fkidcargo)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(user.first_name.as_str())
        .bind(user.last_name.as_str())
        .bind(user.cpf.as_str())
        .bind(user.birth_date)
        .bind(user.username.as_str())
        .bind(user.password_hash.as_str())
        .bind(user.role_id)
        .execute(pool)
        .await?;
        
        // Usa last_insert_id() do resultado para obter o ID corretamente
        let id = result.last_insert_id() as i32;
        Ok(id)
    }

    pub async fn list(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
        let user_rows = sqlx::query(
            r#"
            SELECT id, nome, sobrenome, cpf, datanascimento, user, senha, fkidcargo
            FROM usuario
            "#
        )
        .fetch_all(pool)
        .await?;

        let mut users = Vec::new();
        for row in user_rows {
            use crate::models::user::UserType;

            let role_id: i16 = row.get("fkidcargo");
            let user_id: u64 = row.get("id");
            let user_type = match role_id {
                1 => UserType::Admin,
                2 => UserType::Funcionario,
                3 => UserType::Gerente,
                _ => UserType::Funcionario,
            };

            users.push(User {
                id: Some(user_id as i32),
                first_name: row.get("nome"),
                last_name: row.get("sobrenome"),
                cpf: row.get("cpf"),
                birth_date: row.get("datanascimento"),
                username: row.get("user"),
                password_hash: row.get("senha"),
                user_type,
                role_id,
            });
        }
        Ok(users)
    }


    pub async fn update(pool: &MySqlPool, user: &User) -> Result<(), sqlx::Error> {
        if let Some(user_id) = user.id {
            sqlx::query(
                r#"
                UPDATE usuario
                SET nome = ?, sobrenome = ?, cpf = ?, datanascimento = ?,
                    user = ?, senha = ?, fkidcargo = ?
                WHERE id = ?
                "#,
            )
            .bind(user.first_name.as_str())
            .bind(user.last_name.as_str())
            .bind(user.cpf.as_str())
            .bind(user.birth_date)
            .bind(user.username.as_str())
            .bind(user.password_hash.as_str())
            .bind(user.role_id)
            .bind(user_id)
            .execute(pool)
            .await?;
            Ok(())
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    pub async fn delete(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM usuario WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
