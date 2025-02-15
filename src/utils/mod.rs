/// Collection of helper structs for creating [`Interface`]/[`Peer`]
mod amnezia;
mod keys;

use thiserror::Error;

pub use amnezia::*;
pub use keys::*;

#[derive(Error, Debug)]
pub enum WireguardError {
    #[error("invalid private key")]
    InvalidPrivateKey,

    #[error("invalid public key")]
    InvalidPublicKey,
    
    #[error("no private key provided")]
    NoPrivateKeyProvided,

    #[error("no assigned ip")]
    NoAssignedIP
}

pub type WireguardResult<T> = Result<T, WireguardError>;
