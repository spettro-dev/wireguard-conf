/// Collection of helper structs for creating [`Interface`]/[`Peer`]
mod amnezia;
mod keys;
mod traits;

use thiserror::Error;

pub use amnezia::*;
pub use keys::*;
pub use traits::*;

#[derive(Error, Debug)]
pub enum WireguardError {
    #[error("invalid private key")]
    InvalidPrivateKey,

    #[error("invalid public key")]
    InvalidPublicKey,
    
    #[error("no private key provided")]
    NoPrivateKeyProvided,
}

pub type WireguardResult<T> = Result<T, WireguardError>;
