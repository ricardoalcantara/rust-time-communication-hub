use crate::repository::{
    models::group::{Group, UpdateGroup},
    repository_base::{RepositoryBase, RepositoryResult},
};
use async_trait::async_trait;

#[async_trait]
pub trait GroupRepository {
    async fn group_insert(&self, group: &Group) -> RepositoryResult<i32>;
    async fn group_insert_if_not_exists(&self, group: &Group) -> RepositoryResult<i32>;
    async fn group_exists(&self, name: &str) -> RepositoryResult<Option<i32>>;
    async fn group_delete(&self, id: i32) -> RepositoryResult<bool>;
    async fn group_update(&self, group: &UpdateGroup) -> RepositoryResult<bool>;
    async fn group_select(&self, id: i32) -> RepositoryResult<Option<Group>>;
    async fn group_attach_message(
        &self,
        group_name: &str,
        message_id: i32,
    ) -> RepositoryResult<u64>;
}

#[cfg(feature = "mysql")]
#[async_trait]
impl GroupRepository for RepositoryBase {
    async fn group_insert(&self, group: &Group) -> RepositoryResult<i32> {
        let group_id = sqlx::query!(
            r#"
    INSERT INTO `group` ( name )
    VALUES ( ? )
            "#,
            group.name,
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();

        Ok(group_id as i32)
    }

    async fn group_insert_if_not_exists(&self, group: &Group) -> RepositoryResult<i32> {
        if let Some(group_id) = self.group_exists(&group.name).await? {
            return Ok(group_id);
        }

        self.group_insert(group).await
    }

    async fn group_exists(&self, name: &str) -> RepositoryResult<Option<i32>> {
        let row = sqlx::query!("SELECT id FROM `group` WHERE name = ?", name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| r.id))
    }

    async fn group_delete(&self, id: i32) -> RepositoryResult<bool> {
        let rows_affected = sqlx::query!("DELETE FROM `group` WHERE id = ?", id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    async fn group_update(&self, group: &UpdateGroup) -> RepositoryResult<bool> {
        let rows_affected = sqlx::query!(
            r#"
    UPDATE `group`
    SET name = ?
    WHERE id = ?
            "#,
            group.name,
            group.id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    async fn group_select(&self, id: i32) -> RepositoryResult<Option<Group>> {
        let row = sqlx::query_as!(Group, "SELECT * FROM `group` WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row)
    }

    async fn group_attach_message(
        &self,
        group_name: &str,
        message_id: i32,
    ) -> RepositoryResult<u64> {
        let rows_affected = sqlx::query!(
            r#"
            INSERT INTO user_message (user_id, message_id) 
            SELECT u.id, ? FROM `user` u
            INNER JOIN `user_group` ug ON ug.user_id = u.id 
            INNER JOIN `group` g  ON g.id  = ug.group_id
            WHERE g.name = ?
            "#,
            message_id,
            group_name,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }
}

#[cfg(feature = "postgres")]
#[async_trait]
impl GroupRepository for RepositoryBase {
    async fn group_insert(&self, group: &Group) -> RepositoryResult<i32> {
        let group_id = sqlx::query!(
            r#"
    INSERT INTO `group` ( name )
    VALUES ( ? )
            "#,
            group.name,
        )
        .execute(&self.pool)
        .await?
        .last_insert_id();

        Ok(group_id as i32)
    }

    async fn group_insert_if_not_exists(&self, group: &Group) -> RepositoryResult<i32> {
        if let Some(group_id) = self.group_exists(&group.name).await? {
            return Ok(group_id);
        }

        self.group_insert(group).await
    }

    async fn group_exists(&self, name: &str) -> RepositoryResult<Option<i32>> {
        let row = sqlx::query!("SELECT id FROM `group` WHERE name = ?", name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| r.id))
    }

    async fn group_delete(&self, id: i32) -> RepositoryResult<bool> {
        let rows_affected = sqlx::query!("DELETE FROM `group` WHERE id = ?", id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    async fn group_update(&self, group: &UpdateGroup) -> RepositoryResult<bool> {
        let rows_affected = sqlx::query!(
            r#"
    UPDATE `group`
    SET name = ?
    WHERE id = ?
            "#,
            group.name,
            group.id,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    async fn group_select(&self, id: i32) -> RepositoryResult<Option<Group>> {
        let row = sqlx::query_as!(Group, "SELECT * FROM `group` WHERE id = ?", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row)
    }

    async fn group_attach_message(
        &self,
        group_name: &str,
        message_id: i32,
    ) -> RepositoryResult<u64> {
        let rows_affected = sqlx::query!(
            r#"
            INSERT INTO user_message (user_id, message_id) 
            SELECT u.id, ? FROM `user` u
            INNER JOIN `user_group` ug ON ug.user_id = u.id 
            INNER JOIN `group` g  ON g.id  = ug.group_id
            WHERE g.name = ?
            "#,
            message_id,
            group_name,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected)
    }
}
