use either::Either;
use ipnet::Ipv4Net;

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
