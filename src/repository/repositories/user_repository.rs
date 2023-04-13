use crate::repository::{
    models::user::User,
    repository_base::{RepositoryBase, RepositoryResult},
};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn user_insert(&self, user: &User) -> RepositoryResult<i32>;
    async fn user_insert_if_not_exists(&self, user: &User) -> RepositoryResult<i32>;
    async fn user_get_id(&self, external_id: &str) -> RepositoryResult<Option<i32>>;
    async fn user_delete(&self, id: i32) -> RepositoryResult<bool>;
    // async fn user_update(&self, user: &UpdateUser) -> RepositoryResult<bool>;
    async fn user_select(&self, id: i32) -> RepositoryResult<Option<User>>;
    async fn user_attach_message(&self, id: i32, message_id: i32) -> RepositoryResult<u64>;
    async fn user_attach_group(&self, id: i32, group_id: i32) -> RepositoryResult<u64>;
}

#[cfg(feature = "mysql")]
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
        if let Some(user_id) = self.user_get_id(&user.external_id).await? {
            return Ok(user_id);
        }

        self.user_insert(user).await
    }

    async fn user_get_id(&self, external_id: &str) -> RepositoryResult<Option<i32>> {
        let row = sqlx::query!("SELECT id FROM user WHERE external_id = ?", external_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| r.id))
    }

    async fn user_delete(&self, id: i32) -> RepositoryResult<bool> {
        let rows_affected = sqlx::query!("DELETE FROM user WHERE id = ?", id)
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

    async fn user_attach_message(&self, id: i32, message_id: i32) -> RepositoryResult<u64> {
        let rows_affected = sqlx::query!(
            r#"
            INSERT INTO user_message (user_id, message_id)
            VALUES ( ? ,? )
            "#,
            id,
            message_id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }

    async fn user_attach_group(&self, id: i32, group_id: i32) -> RepositoryResult<u64> {
        let rows_affected = sqlx::query!(
            r#"
            INSERT INTO user_group (user_id, group_id)
            VALUES ( ? ,? )
            "#,
            id,
            group_id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }
}

#[cfg(feature = "postgres")]
#[async_trait]
impl UserRepository for RepositoryBase {
    async fn user_insert(&self, user: &User) -> RepositoryResult<i32> {
        let rec = sqlx::query!(
            r#"
    INSERT INTO public.user ( external_id, created_at )
    VALUES ( $1, $2 )
    RETURNING id
            "#,
            user.external_id,
            user.created_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec.id as i32)
    }

    async fn user_insert_if_not_exists(&self, user: &User) -> RepositoryResult<i32> {
        if let Some(user_id) = self.user_get_id(&user.external_id).await? {
            return Ok(user_id);
        }

        self.user_insert(user).await
    }

    async fn user_get_id(&self, external_id: &str) -> RepositoryResult<Option<i32>> {
        let row = sqlx::query!(
            "SELECT id FROM public.user WHERE external_id = $1",
            external_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.id))
    }

    async fn user_delete(&self, id: i32) -> RepositoryResult<bool> {
        let rows_affected = sqlx::query!("DELETE FROM public.user WHERE id = $1", id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    async fn user_select(&self, id: i32) -> RepositoryResult<Option<User>> {
        let row = sqlx::query_as!(User, "SELECT * FROM public.user WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row)
    }

    async fn user_attach_message(&self, id: i32, message_id: i32) -> RepositoryResult<u64> {
        let rows_affected = sqlx::query!(
            r#"
            INSERT INTO public.user_message (user_id, message_id)
            VALUES ( $1, $2 )
            "#,
            id,
            message_id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }

    async fn user_attach_group(&self, id: i32, group_id: i32) -> RepositoryResult<u64> {
        let rows_affected = sqlx::query!(
            r#"
            INSERT INTO public.user_group (user_id, group_id)
            VALUES ( $1, $2 )
            "#,
            id,
            group_id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }
}

#[cfg(feature = "sqlite")]
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
        .last_insert_rowid();

        Ok(user_id as i32)
    }

    async fn user_insert_if_not_exists(&self, user: &User) -> RepositoryResult<i32> {
        if let Some(user_id) = self.user_get_id(&user.external_id).await? {
            return Ok(user_id);
        }

        self.user_insert(user).await
    }

    async fn user_get_id(&self, external_id: &str) -> RepositoryResult<Option<i32>> {
        let row = sqlx::query!(
            r#"SELECT id as "id!" FROM user WHERE external_id = ?"#,
            external_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.id as i32))
    }

    async fn user_delete(&self, id: i32) -> RepositoryResult<bool> {
        let rows_affected = sqlx::query!("DELETE FROM user WHERE id = ?", id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    async fn user_select(&self, id: i32) -> RepositoryResult<Option<User>> {
        let row = sqlx::query_as!(
            User,
            r#"SELECT id as "id:i32", external_id, created_at FROM user WHERE id = ?"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    async fn user_attach_message(&self, id: i32, message_id: i32) -> RepositoryResult<u64> {
        let rows_affected = sqlx::query!(
            r#"
            INSERT INTO user_message (user_id, message_id)
            VALUES ( ? ,? )
            "#,
            id,
            message_id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }

    async fn user_attach_group(&self, id: i32, group_id: i32) -> RepositoryResult<u64> {
        let rows_affected = sqlx::query!(
            r#"
            INSERT INTO user_group (user_id, group_id)
            VALUES ( ? ,? )
            "#,
            id,
            group_id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }
}
