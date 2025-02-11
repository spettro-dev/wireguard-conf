use core::fmt;

use base64::prelude::*;
use x25519_dalek::{PublicKey as XPublicKey, StaticSecret};

use crate::WireguardError;

#[derive(Clone)]
pub struct PrivateKey {
    secret: StaticSecret,
}

impl PrivateKey {
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

#[derive(Clone)]
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

impl ToString for PublicKey {
    fn to_string(&self) -> String {
        BASE64_STANDARD.encode(self.key.to_bytes())
    }
}
