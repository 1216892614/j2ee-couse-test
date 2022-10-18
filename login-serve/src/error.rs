use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Mongod data base error")]
    Mongod(#[from] mongod::Error),
    #[error("The user name intended to insert exists")]
    UserNameExists(String),
}
