use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("mongo data base error")]
    DataBase(#[from] mongod::Error),
    #[error("unknown data store error")]
    Unknown,
}
