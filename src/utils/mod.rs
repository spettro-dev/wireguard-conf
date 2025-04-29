#[cfg(feature = "amneziawg")]
mod amnezia;
mod keys;

use thiserror::Error;

#[cfg(feature = "amneziawg")]
#[cfg_attr(docsrs, doc(cfg(feature = "amneziawg")))]
pub use amnezia::*;
pub use keys::*;

#[derive(Error, Debug, PartialEq)]
pub enum WireguardError {
    #[error("invalid private key")]
    InvalidPrivateKey,

    #[error("invalid public key")]
    InvalidPublicKey,

    #[error("invalid preshared key")]
    InvalidPresharedKey,

    #[error("no private key provided")]
    NoPrivateKeyProvided,

    #[error("no assigned ip")]
    NoAssignedIP,

    #[cfg(feature = "amneziawg")]
    #[error("invalid amnezia setting: {0}")]
    InvalidAmneziaSetting(String),
}

pub type WireguardResult<T> = Result<T, WireguardError>;
