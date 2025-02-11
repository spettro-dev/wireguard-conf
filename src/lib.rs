use thiserror::Error;

pub mod models;
pub mod prelude;
pub mod utils;

#[derive(Error, Debug)]
pub enum WireguardError {
    #[error("invalid private key")]
    InvalidPrivateKey,

    #[error("invalid public key")]
    InvalidPublicKey,
}
