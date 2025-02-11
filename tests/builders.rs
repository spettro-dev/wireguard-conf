use wireguard_conf::prelude::*;

#[test]
fn builders() {
    let address = "10.3.2.1/24".parse().unwrap();

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
