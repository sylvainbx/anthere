use std::fmt;
use serde::de::StdError;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum AppError {
    InternalServerError,
    NotFound,
}

impl std::error::Error for AppError {}

// Utility function to adapt errors of generic type T into AppError
pub fn adapt_app_error<T: Error>(error: T) -> AppError {
    error.as_app_error()
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Not found"),
            AppError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

pub trait Error {
    fn as_app_error(&self) -> AppError;
}

impl Error for diesel::result::Error {
    fn as_app_error(&self) -> AppError {
        tracing::error!("{}", self);
        match self {
            diesel::result::Error::NotFound => AppError::NotFound,
            _ => AppError::InternalServerError,
        }
    }
}

impl Error for r2d2::Error {
    fn as_app_error(&self) -> AppError {
        tracing::error!("{}", self);
        match self {
            _ => AppError::InternalServerError
        }
    }
}

impl Error for JoinError {
    fn as_app_error(&self) -> AppError {
        tracing::error!("{}", self);
        AppError::InternalServerError
    }
}

impl Error for &str {
    fn as_app_error(&self) -> AppError {
        tracing::error!("{}", self);
        AppError::InternalServerError
    }
}

impl Error for Box<dyn StdError + Send + Sync> {
    fn as_app_error(&self) -> AppError {
        tracing::error!("{}", self);
        AppError::InternalServerError
    }
}