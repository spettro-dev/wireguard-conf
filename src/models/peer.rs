use either::Either;
use ipnet::Ipv4Net;

use std::fmt;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Peer {
    /// Peer's endpoint.
    pub endpoint: Option<String>,

    /// Peer's allowed IPs.
    pub allowed_ips: Vec<Ipv4Net>,

    /// Peer's key.
    ///
    /// If [`PrivateKey`] is provided, then peer can be exported to interface & full config.
    /// Otherwise only to peer section of config.
    pub key: Either<PrivateKey, PublicKey>,
}

impl Peer {
    /// Get Peer's [`Interface`].
    ///
    /// Pass server's interface to `interface` argument.
    ///
    /// # Errors
    ///
    /// - [`WireguardError::NoPrivateKeyProvided`] -- peer don't have private key.
    ///   You need to provide [`PrivateKey`] for creating interfaces from peers.
    pub fn as_interface(&self, interface: &Interface) -> WireguardResult<Interface> {
        let Either::Left(private_key) = self.key.clone() else {
            return Err(WireguardError::NoPrivateKeyProvided);
        };

        let assigned_ip = self
            .allowed_ips
            .iter()
            .find(|&net| interface.address.contains(net))
            .unwrap()
            .clone();

        Ok(Interface {
            address: assigned_ip,
            listen_port: None,
            private_key,
            dns: interface.dns.clone(),
            // amnezia_settings: None,
            endpoint: None,
            peers: vec![interface.as_peer()],
        })
    }
}

impl fmt::Display for Peer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[Peer]")?;
        if let Some(endpoint) = self.endpoint.clone() {
            writeln!(f, "Endpoint = {}", endpoint)?;
        }
        writeln!(
            f,
            "AllowedIPs = {}",
            self.allowed_ips
                .iter()
                .map(|net| net.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )?;
        writeln!(
            f,
            "PublicKey = {}",
            self.key.clone().right_or_else(|key| PublicKey::from(&key))
        )?;

        Ok(())
    }
}
