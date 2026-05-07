use sqlx::MySqlPool;
use sqlx::Row;
use crate::models::user::{User, UserType};

pub struct UserRepository;

impl UserRepository {
    pub async fn create(pool: &MySqlPool, user: &User) -> Result<i32, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO users (username, password_hash, first_name, last_name, birth_date, cpf, role_id)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&user.username.as_str())
        .bind(&user.password_hash.as_str())
        .bind(&user.first_name.as_str())
        .bind(&user.last_name.as_str())
        .bind(&user.birth_date)
        .bind(&user.cpf.as_str())
        .bind(&user.role_id)
        .execute(pool)
        .await?;
        
        // Usa last_insert_id() do resultado para obter o ID corretamente
        let id = result.last_insert_id() as i32;
        Ok(id)
    }

    pub async fn list(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
        let user_rows = sqlx::query(
            r#"
            SELECT id, first_name, last_name, cpf, birth_date, username, password_hash, role_id
            FROM users
            "#
        )
        .fetch_all(pool)
        .await?;

        let mut users = Vec::new();
        for row in user_rows {
            use crate::models::user::UserType;

            let role_id: i16 = row.get("role_id");
            let user_id: i32 = row.get("id");
            let user_type = match role_id {
                1 => UserType::Admin,
                2 => UserType::Funcionario,
                3 => UserType::Gerente,
                _ => UserType::Funcionario,
            };

            users.push(User {
                id: Some(user_id),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                birth_date: row.get("birth_date"),
                cpf: row.get("cpf"),
                username: row.get("username"),
                password_hash: row.get("password_hash"),
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
            UPDATE users
            SET first_name = ?, last_name = ?, birth_date = ?,
                username = ?, password_hash = ?, role_id = ?, cpf = ?
            WHERE id = ?
            "#,
        )
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(user.birth_date)
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(user.role_id)
        .bind(&user.cpf)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}

    pub async fn delete(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    // Buscar por ID
    pub async fn get_by_id(pool: &MySqlPool, id: i32) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, username, first_name, last_name, cpf, birth_date, password_hash, role_id
            FROM users WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(pool) // Retorna Option para caso o ID não exista
        .await?;

        if let Some(row) = row {
            let role_id: i16 = row.get("role_id");
            let user_type = match role_id {
                1 => UserType::Admin,
                2 => UserType::Funcionario,
                3 => UserType::Gerente,
                _ => UserType::Funcionario,
            };

            Ok(Some(User {
                id: Some(row.get("id")),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                birth_date: row.get("birth_date"),
                cpf: row.get("cpf"),
                username: row.get("username"),
                password_hash: row.get("password_hash"),
                user_type,
                role_id,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn promote_user(pool: &MySqlPool, user_id: i32, new_role_id: i16) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET role_id = ? WHERE id = ?")
            .bind(new_role_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

}
