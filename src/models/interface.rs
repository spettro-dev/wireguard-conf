use ipnet::Ipv4Net;

use std::fmt::Write;

use crate::{prelude::*, utils::amnezia::AmneziaSettings};

#[derive(Clone, Debug)]
pub struct Interface {
    pub address: Ipv4Net,
    pub listen_port: Option<u16>,
    pub private_key: PrivateKey,
    pub dns: Vec<String>,
    pub amnezia_settings: Option<AmneziaSettings>,

    pub peers: Vec<Peer>,
}

impl Interface {
    fn add_peer(&mut self) {}
}

impl ExportConfig for Interface {
    fn config(&self) -> String {
        let mut config = String::new();

        writeln!(&mut config, "[Interface]").unwrap();
        writeln!(&mut config, "Address = {}", self.address).unwrap();
        if let Some(listen_port) = self.listen_port {
            writeln!(&mut config, "ListenPort = {}", listen_port).unwrap();
        }
        writeln!(&mut config, "PrivateKey = {}", self.private_key).unwrap();
        if !self.dns.is_empty() {
            writeln!(&mut config, "DNS = {}", self.dns.join(",")).unwrap();
        }

        config
    }
}
