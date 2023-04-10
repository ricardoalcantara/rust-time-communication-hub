use crate::repository::repository_base::RepositoryBase;
use async_trait::async_trait;

#[async_trait]
pub trait MessageRepository {}

#[async_trait]
impl MessageRepository for RepositoryBase {}
