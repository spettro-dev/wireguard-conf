#![cfg(feature = "amneziawg")]

use wireguard_conf::prelude::*;

#[test]
fn amnezia_random() {
    let amnezia_settings = AmneziaSettings::random();

    assert_eq!(amnezia_settings.validate(), Ok(()));

    _ = InterfaceBuilder::new()
        .amnezia_settings(amnezia_settings)
        .build();
}

#[test]
fn amnezia_validation() {
    let amnezia_settings = AmneziaSettings::random();

    {
        let settings = AmneziaSettings {
            jc: 999,
            ..amnezia_settings
        };

        assert_eq!(
            settings.validate(),
            Err(WireguardError::InvalidAmneziaSetting("Jc".to_string()))
        );
    }

    {
        let settings = AmneziaSettings {
            jmin: 800,
            jmax: 500,
            ..amnezia_settings
        };

        assert_eq!(
            settings.validate(),
            Err(WireguardError::InvalidAmneziaSetting("Jmin".to_string()))
        );
    }

    {
        let settings = AmneziaSettings {
            jmax: 9999,
            ..amnezia_settings
        };

        assert_eq!(
            settings.validate(),
            Err(WireguardError::InvalidAmneziaSetting("Jmax".to_string()))
        );
    }

    {
        let settings = AmneziaSettings {
            s1: 9999,
            ..amnezia_settings
        };

        assert_eq!(
            settings.validate(),
            Err(WireguardError::InvalidAmneziaSetting("S1".to_string()))
        );
    }

    {
        let settings = AmneziaSettings {
            s1: 100 - 56,
            s2: 100,
            ..amnezia_settings
        };

        assert_eq!(
            settings.validate(),
            Err(WireguardError::InvalidAmneziaSetting("S1".to_string()))
        );
    }

    {
        let settings = AmneziaSettings {
            s2: 9999,
            ..amnezia_settings
        };

        assert_eq!(
            settings.validate(),
            Err(WireguardError::InvalidAmneziaSetting("S2".to_string()))
        );
    }

    {
        let settings = AmneziaSettings {
            h1: 222,
            h2: 222,
            h3: 333,
            h4: 444,
            ..amnezia_settings
        };

        assert_eq!(
            settings.validate(),
            Err(WireguardError::InvalidAmneziaSetting(
                "H1/H2/H3/H4".to_string()
            ))
        );
    }
}
