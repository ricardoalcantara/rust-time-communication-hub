use crate::repository::{
    models::user::User,
    repository_base::{RepositoryBase, RepositoryResult},
};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn user_insert(&self, user: &User) -> RepositoryResult<i32>;
    async fn user_insert_if_not_exists(&self, user: &User) -> RepositoryResult<i32>;
    async fn user_exists(&self, external_id: &str) -> RepositoryResult<Option<i32>>;
    async fn user_delete(&self, id: i32) -> RepositoryResult<bool>;
    async fn user_update(&self, user: &User) -> RepositoryResult<bool>;
    async fn user_select(&self, id: i32) -> RepositoryResult<Option<User>>;
}

#[async_trait]
impl UserRepository for RepositoryBase {
    async fn user_insert(&self, user: &User) -> RepositoryResult<i32> {
        let user_id = sqlx::query!(
            r#"
    INSERT INTO user ( external_id, created_at )
    VALUES ( ?, ? )
            "#,
            user.external_id,
            user.created_at
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();

        Ok(user_id as i32)
    }

    async fn user_insert_if_not_exists(&self, user: &User) -> RepositoryResult<i32> {
        if let Some(user_id) = self.user_exists(&user.external_id).await? {
            return Ok(user_id);
        }

        self.user_insert(user).await
    }

    async fn user_exists(&self, external_id: &str) -> RepositoryResult<Option<i32>> {
        let row: Option<(i32,)> = sqlx::query_as("SELECT id FROM user WHERE external_id = ?")
            .bind(external_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| r.0))
    }

    async fn user_delete(&self, id: i32) -> RepositoryResult<bool> {
        let rows_affected = sqlx::query!("DELETE FROM user WHERE id = ?", id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    async fn user_update(&self, user: &User) -> RepositoryResult<bool> {
        let rows_affected = sqlx::query!(
            r#"
    UPDATE user
    SET external_id = ?
    WHERE id = ?
            "#,
            user.external_id,
            user.id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    async fn user_select(&self, id: i32) -> RepositoryResult<Option<User>> {
        let row = sqlx::query_as!(User, "SELECT * FROM user WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row)
    }
}
