use std::{convert::Infallible, panic::Location};
use thiserror::Error;

use crate::repository::repository_base::RepositoryError;

pub type AppResult<T = ()> = anyhow::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IOError: {error}; Location: {location}")]
    IOError {
        error: std::io::Error,
        location: &'static Location<'static>,
    },
    #[error("HyperError: {error}; Location: {location}")]
    HyperError {
        error: hyper::Error,
        location: &'static Location<'static>,
    },
    #[error("FromStrError: {error}; Location: {location}")]
    FromStrError {
        error: String,
        location: &'static Location<'static>,
    },
    #[error(transparent)]
    Infallible(#[from] Infallible),
    #[error(transparent)]
    RepositoryError(#[from] RepositoryError),
}

impl From<std::io::Error> for AppError {
    #[track_caller]
    fn from(error: std::io::Error) -> Self {
        AppError::IOError {
            error,
            location: Location::caller(),
        }
    }
}

impl From<axum::http::header::InvalidHeaderValue> for AppError {
    #[track_caller]
    fn from(error: axum::http::header::InvalidHeaderValue) -> Self {
        AppError::FromStrError {
            error: format!("{error:?}"),
            location: Location::caller(),
        }
    }
}

impl From<hyper::Error> for AppError {
    #[track_caller]
    fn from(error: hyper::Error) -> Self {
        AppError::HyperError {
            error,
            location: Location::caller(),
        }
    }
}
