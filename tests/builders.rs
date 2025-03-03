use ipnet::Ipv4Net;
use wireguard_conf::{as_ipnet, prelude::*};

#[test]
fn interface_builder() {
    let address = as_ipnet!("10.3.2.1/24");

    let interface = InterfaceBuilder::new()
        .address(address)
        .listen_port(55870)
        .set_dns(vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()])
        .add_dns("1.1.1.1".to_string())
        .add_peer(
            PeerBuilder::new()
                .set_allowed_ips(vec!["0.0.0.0/0".parse().unwrap()])
                .add_allowed_ip("10.3.2.2/24".parse().unwrap())
                .public_key(PublicKey::from(&PrivateKey::random()))
                .build(),
        )
        .build();

    assert_eq!(interface.address, address);
    assert_eq!(interface.listen_port, Some(55870));
    assert_eq!(interface.dns.len(), 3);
    assert_eq!(interface.peers.len(), 1);
}

#[test]
fn peer_builder() {
    let allowed_ip = as_ipnet!("10.3.2.1/32");
    let endpoint = "peer.example.com".to_string();

    let peer = PeerBuilder::new()
        .set_allowed_ips(vec![allowed_ip])
        .endpoint(endpoint.clone())
        .build();

    assert_eq!(peer.allowed_ips, vec![allowed_ip]);
    assert_eq!(peer.endpoint, Some(endpoint));
}
