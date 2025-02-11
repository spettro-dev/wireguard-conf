use either::Either;
use ipnet::Ipv4Net;

use crate::prelude::*;

#[derive(Default)]
pub struct InterfaceBuilder {
    address: Ipv4Net,
    listen_port: Option<u16>,
    private_key: Option<PrivateKey>,
    dns: Vec<String>,
    peers: Vec<Peer>,
}

impl InterfaceBuilder {
    pub fn new() -> InterfaceBuilder {
        InterfaceBuilder::default()
    }

    pub fn address(mut self, address: Ipv4Net) -> Self {
        self.address = address;
        self
    }

    pub fn listen_port(mut self, listen_port: u16) -> Self {
        self.listen_port = Some(listen_port);
        self
    }

    pub fn private_key(mut self, private_key: PrivateKey) -> Self {
        self.private_key = Some(private_key);
        self
    }

    pub fn set_dns(mut self, dns: Vec<String>) -> Self {
        self.dns = dns;
        self
    }

    pub fn add_dns(mut self, dns: String) -> Self {
        self.dns.push(dns);
        self
    }

    pub fn set_peers(mut self, peers: Vec<Peer>) -> Self {
        self.peers = peers;
        self
    }

    pub fn add_peer(mut self, peer: Peer) -> Self {
        self.peers.push(peer);
        self
    }

    pub fn build(self) -> Interface {
        Interface {
            address: self.address,
            listen_port: self.listen_port,
            private_key: self.private_key.unwrap_or_else(|| PrivateKey::random()),
            dns: self.dns,

            // TODO: do amnezia support
            amnezia_settings: None,

            peers: self.peers,
        }
    }
}

#[derive(Default)]
pub struct PeerBuilder {
    endpoint: Option<String>,
    allowed_ips: Vec<Ipv4Net>,
    key: Option<Either<PrivateKey, PublicKey>>,
}

impl PeerBuilder {
    pub fn new() -> PeerBuilder {
        PeerBuilder::default()
    }

    pub fn endpoint(mut self, endpoint: String) -> PeerBuilder {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn set_allowed_ips(mut self, ip: Vec<Ipv4Net>) -> PeerBuilder {
        self.allowed_ips = ip;
        self
    }

    pub fn add_allowed_ip(mut self, ip: Ipv4Net) -> PeerBuilder {
        self.allowed_ips.push(ip);
        self
    }

    pub fn private_key(mut self, private_key: PrivateKey) -> PeerBuilder {
        self.key = Some(Either::Left(private_key));
        self
    }

    pub fn public_key(mut self, public_key: PublicKey) -> PeerBuilder {
        self.key = Some(Either::Right(public_key));
        self
    }

    pub fn build(self) -> Peer {
        let key = self
            .key
            .unwrap_or_else(|| Either::Left(PrivateKey::random()));

        Peer {
            endpoint: self.endpoint,
            allowed_ips: self.allowed_ips,
            key,
        }
    }
}
