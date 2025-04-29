use either::Either;
use ipnet::Ipv4Net;

use std::fmt;

use crate::prelude::*;

#[must_use]
#[derive(Clone, Debug)]
pub struct Interface {
    pub address: Ipv4Net,
    pub listen_port: Option<u16>,
    pub private_key: PrivateKey,
    pub dns: Vec<String>,
    pub endpoint: Option<String>,

    #[cfg(feature = "amneziawg")]
    #[cfg_attr(docsrs, doc(cfg(feature = "amneziawg")))]
    pub amnezia_settings: Option<AmneziaSettings>,

    pub peers: Vec<Peer>,
}

impl Interface {
    pub fn to_peer(&self) -> Peer {
        Peer {
            endpoint: self.endpoint.clone(),
            allowed_ips: vec![self.address],
            key: Either::Left(self.private_key.clone()),
            preshared_key: None,
            persistent_keepalive: None,

            #[cfg(feature = "amneziawg")]
            amnezia_settings: self.amnezia_settings.clone(),
        }
    }
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[Interface]")?;
        if let Some(endpoint) = &self.endpoint {
            writeln!(f, "# Name = {endpoint}")?;
        }
        writeln!(f, "Address = {}", self.address)?;
        if let Some(listen_port) = self.listen_port {
            writeln!(f, "ListenPort = {listen_port}")?;
        }
        writeln!(f, "PrivateKey = {}", self.private_key)?;
        if !self.dns.is_empty() {
            writeln!(f, "DNS = {}", self.dns.join(","))?;
        }

        #[cfg(feature = "amneziawg")]
        if let Some(amnezia_settings) = &self.amnezia_settings {
            writeln!(f)?;
            writeln!(f, "{amnezia_settings}")?;
        }

        for peer in &self.peers {
            writeln!(f)?;
            writeln!(f, "{peer}")?;
        }

        fmt::Result::Ok(())
    }
}
