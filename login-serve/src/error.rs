use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Mongod data base error: {0}")]
    Mongod(#[from] mongod::Error),
    #[error("Cannt find a user with this username: {0}")]
    WrongUsername(String),
    #[error("The user name: {0} intended to insert exists")]
    UserNameExists(String),
}
