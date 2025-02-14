# Wireguard Conf

Easy to use library for managing wireguard configs.

## Installation

```shell
...
```

### Usage

```rust
use wireguard_conf::prelude::*;
use wireguard_conf::as_ipnet;

use ipnet::Ipv4Net;

// create peer:
let peer = PeerBuilder::new()
    .add_allowed_ip(as_ipnet!("10.0.0.2/24"))
    .build();

// create interface with that peer:
let interface = InterfaceBuilder::new()
    .address(as_ipnet!("10.0.0.1/24"))
    .add_peer(peer.clone())
    .build();

println!("Server's config:");
println!("{}\n", interface);

println!("Client's config:");
println!("{}", peer.as_interface(&interface).unwrap());
```
