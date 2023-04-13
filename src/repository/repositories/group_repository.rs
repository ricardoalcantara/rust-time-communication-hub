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
        let rec = sqlx::query!(
            r#"
    INSERT INTO public.group ( name )
    VALUES ( $1 )
    RETURNING id
            "#,
            group.name,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec.id as i32)
    }

    async fn group_insert_if_not_exists(&self, group: &Group) -> RepositoryResult<i32> {
        if let Some(group_id) = self.group_exists(&group.name).await? {
            return Ok(group_id);
        }

        self.group_insert(group).await
    }

    async fn group_exists(&self, name: &str) -> RepositoryResult<Option<i32>> {
        let row = sqlx::query!("SELECT id FROM public.group WHERE name = $1", name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| r.id))
    }

    async fn group_delete(&self, id: i32) -> RepositoryResult<bool> {
        let rows_affected = sqlx::query!("DELETE FROM public.group WHERE id = $1", id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    async fn group_update(&self, group: &UpdateGroup) -> RepositoryResult<bool> {
        let rows_affected = sqlx::query!(
            r#"
    UPDATE public.group
    SET name = $1
    WHERE id = $2
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
        let row = sqlx::query_as!(Group, "SELECT * FROM public.group WHERE id = $1", id)
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
            SELECT u.id, $1 FROM public.user u
            INNER JOIN public.user_group ug ON ug.user_id = u.id 
            INNER JOIN public.group g  ON g.id  = ug.group_id
            WHERE g.name = $2
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

#[cfg(feature = "sqlite")]
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
        .last_insert_rowid();

        Ok(group_id as i32)
    }

    async fn group_insert_if_not_exists(&self, group: &Group) -> RepositoryResult<i32> {
        if let Some(group_id) = self.group_exists(&group.name).await? {
            return Ok(group_id);
        }

        self.group_insert(group).await
    }

    async fn group_exists(&self, name: &str) -> RepositoryResult<Option<i32>> {
        let row = sqlx::query!(r#"SELECT id  as "id!" FROM `group` WHERE name = ?"#, name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| r.id as i32))
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
        let row = sqlx::query_as!(
            Group,
            r#"SELECT id  as "id:i32", name FROM `group` WHERE id = ?"#,
            id
        )
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
