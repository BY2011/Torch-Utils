use thiserror::Error;

/// ## Description
/// This enum describes ownable extension error
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Owna