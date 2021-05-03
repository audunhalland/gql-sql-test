use thiserror::Error;

///
/// The set of errors this application can produce
///
#[derive(Error, Debug)]
pub enum AppError {
    #[error("database")]
    Db(sqlx::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Db(err)
    }
}
