use core::fmt;

use base64::prelude::*;
use rand::RngCore;
use x25519_dalek::{PublicKey as XPublicKey, StaticSecret};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::WireguardError;

/// Private key
///
/// Wrapper around [`x25519_dalek::StaticSecret`]. It can be formatted to Wireguard's
/// format, and also implements [`fmt::Debug`].
#[derive(Clone)]
pub struct PrivateKey {
    secret: StaticSecret,
}

impl PrivateKey {
    #[must_use]
    pub fn random() -> PrivateKey {
        Self {
            secret: StaticSecret::random(),
        }
    }
}

impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrivateKey")
            .field("secret", &self.to_string())
            .finish()
    }
}

impl fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.secret.to_bytes()))
    }
}

impl PartialEq for PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.secret.as_bytes() == other.secret.as_bytes()
    }
}

impl TryFrom<String> for PrivateKey {
    type Error = WireguardError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes: [u8; 32] = BASE64_STANDARD
            .decode(value)
            .map_err(|_| WireguardError::InvalidPrivateKey)?
            .try_into()
            .map_err(|_| WireguardError::InvalidPrivateKey)?;

        Ok(Self {
            secret: StaticSecret::from(bytes),
        })
    }
}

/// Public key.
///
/// Wrapper around [`x25519_dalek::PublicKey`]. It can be formatted to Wireguard's
/// format, and also implements [`fmt::Debug`].
#[derive(Clone, PartialEq)]
pub struct PublicKey {
    key: XPublicKey,
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PublicKey")
            .field("key", &self.to_string())
            .finish()
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.key.to_bytes()))
    }
}

impl TryFrom<String> for PublicKey {
    type Error = WireguardError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes: [u8; 32] = BASE64_STANDARD
            .decode(value)
            .map_err(|_| WireguardError::InvalidPublicKey)?
            .try_into()
            .map_err(|_| WireguardError::InvalidPublicKey)?;

        Ok(Self {
            key: XPublicKey::from(bytes),
        })
    }
}

impl From<&PrivateKey> for PublicKey {
    fn from(value: &PrivateKey) -> Self {
        Self {
            key: XPublicKey::from(&value.secret),
        }
    }
}

/// Preshared key.
///
/// A 32-byte symmetric key used for additional security.
/// Wraps a simple [u8; 32] byte array.
#[derive(Clone, PartialEq, Zeroize, ZeroizeOnDrop)]
pub struct PresharedKey {
    key: [u8; 32],
}

impl PresharedKey {
    /// Generates a new cryptographically secure random preshared key.
    #[must_use]
    pub fn random() -> Self {
        let mut key = [0u8; 32];

        rand::rng().fill_bytes(&mut key);

        Self { key }
    }

    /// Returns a reference to the raw bytes of the preshared key.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.key
    }
}

impl fmt::Debug for PresharedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PresharedKey")
            .field("key", &self.to_string())
            .finish()
    }
}

impl fmt::Display for PresharedKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", BASE64_STANDARD.encode(self.key))
    }
}

impl TryFrom<String> for PresharedKey {
    type Error = WireguardError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes: [u8; 32] = BASE64_STANDARD
            .decode(value.trim())
            .map_err(|_| WireguardError::InvalidPresharedKey)?
            .try_into()
            .map_err(|_| WireguardError::InvalidPresharedKey)?;

        Ok(Self { key: bytes })
    }
}
