use wireguard_conf::prelude::*;

#[test]
fn export_interface_config() {
    let server_key = PrivateKey::random();
    let client_key = PrivateKey::random();

    let server_interface = InterfaceBuilder::new()
        .address("10.0.0.1/24".parse().unwrap())
        .private_key(server_key)
        .add_peer(
            PeerBuilder::new()
                .add_allowed_ip("10.0.0.2/24".parse().unwrap())
                .private_key(client_key)
                .build(),
        )
        .build();
}
