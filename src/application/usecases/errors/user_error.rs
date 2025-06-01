use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Invalid email")]
    InvalidEmail,

    #[error("Password too short")]
    PasswordShort,

    #[error("User already exists")]
    UserExists,

    #[error("Unknown error")]
    Unknown,
}
