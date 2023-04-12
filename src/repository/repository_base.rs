use sqlx::Error as SqlxError;
use std::panic::Location;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("SqlxError: {error}; Location: {location}")]
    SqlxError {
        error: SqlxError,
        location: &'static Location<'static>,
    },
}

impl From<SqlxError> for RepositoryError {
    #[track_caller]
    fn from(error: SqlxError) -> Self {
        RepositoryError::SqlxError {
            error,
            location: Location::caller(),
        }
    }
}

pub type RepositoryResult<T = ()> = Result<T, RepositoryError>;

#[derive(Clone)]
pub struct RepositoryBase {
    #[cfg(feature = "mysql")]
    pub pool: sqlx::MySqlPool,
    #[cfg(feature = "postgres")]
    pub pool: sqlx::MySqlPool,
    #[cfg(feature = "sqlite")]
    pub pool: sqlx::sqlite::SqlitePool,
}

impl RepositoryBase {
    pub async fn new() -> Result<RepositoryBase, RepositoryError> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        #[cfg(feature = "mysql")]
        let pool = sqlx::MySqlPool::connect(&database_url).await?;
        #[cfg(feature = "postgres")]
        let pool = sqlx::postgres::PgPool::connect(&database_url).await?;
        #[cfg(feature = "sqlite")]
        let pool = sqlx::sqlite::SqlitePool::connect(&database_url).await?;

        let repository = RepositoryBase { pool };

        repository.ping().await?;

        Ok(repository)
    }

    pub async fn ping(&self) -> Result<(), RepositoryError> {
        let result = sqlx::query!("SELECT (1) as test")
            .fetch_one(&self.pool)
            .await?;

        assert_eq!(1, result.test);
        tracing::debug!("Database connected");

        Ok(())
    }
}
