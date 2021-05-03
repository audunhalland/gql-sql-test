use std::sync::Arc;
use thiserror::Error;

///
/// The set of errors this application can produce
///
#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("database")]
    Db(Arc<sqlx::Error>),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Db(Arc::new(err))
    }
}
