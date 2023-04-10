use crate::repository::{
    models::message::{Message, UpdateMessage},
    repository_base::{RepositoryBase, RepositoryResult},
};
use async_trait::async_trait;

#[async_trait]
pub trait MessageRepository {
    async fn message_insert(&self, message: &Message) -> RepositoryResult<i32>;
    async fn message_delete(&self, id: i32) -> RepositoryResult<bool>;
    async fn message_update(&self, message: &UpdateMessage) -> RepositoryResult<bool>;
    async fn message_select(&self, id: i32) -> RepositoryResult<Option<Message>>;
}

#[async_trait]
impl MessageRepository for RepositoryBase {
    async fn message_insert(&self, message: &Message) -> RepositoryResult<i32> {
        let message_id = sqlx::query!(
            r#"
    INSERT INTO message ( payload, created_at )
    VALUES ( ?, ? )
            "#,
            message.payload,
            message.created_at
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();

        Ok(message_id as i32)
    }

    async fn message_delete(&self, id: i32) -> RepositoryResult<bool> {
        let rows_affected = sqlx::query!("DELETE FROM message WHERE id = ?", id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    async fn message_update(&self, message: &UpdateMessage) -> RepositoryResult<bool> {
        let rows_affected = sqlx::query!(
            r#"
    UPDATE message
    SET payload = ?
    WHERE id = ?
            "#,
            message.payload,
            message.id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    async fn message_select(&self, id: i32) -> RepositoryResult<Option<Message>> {
        let row = sqlx::query_as!(Message, "SELECT * FROM message WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row)
    }
}
