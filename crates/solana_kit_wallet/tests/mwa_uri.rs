#![cfg(feature = "mwa")]

use solana_kit_wallet::mwa::*;

fn association(host: &str) -> Association {
    let mut scalar = [0; 32];
    scalar[31] = 1;

    Association::Remote {
        key: AssociationKey::from_scalar(&scalar).unwrap().public_key(),
        protocol: "v1 +&=λ".into(),
        host: host.into(),
        id: ReflectorId::new(1).unwrap(),
    }
}

#[test]
fn reflector_authorities_preserve_canonical_spelling() {
    for host in [
        "reflector.example",
        "Reflector.Example:443",
        "[::1]:443",
        "127.0.0.1",
        "r%65flector.example",
    ] {
        let remote = association(host);
        let uri = association_uri(&remote, None).unwrap();
        assert_eq!(parse_association_uri(&uri).unwrap(), remote);
    }

    for host in [
        "",
        "user@evil.example",
        "@evil.example",
        "a/b",
        "a?b",
        "a#b",
        "a\\b",
        "λ.example",
        "a%",
        "a%0G",
        "a:",
        "a:0443",
        "a:65536",
        "a:-1",
        "[0:0:0:0:0:0:0:1]",
        "[::FFFF:192.0.2.1]",
        "[v1.a]",
        "[::1%25zone]",
    ] {
        assert!(
            association_uri(&association(host), None).is_err(),
            "accepted {host}"
        );
    }
}

#[test]
fn wallet_prefix_and_absolute_uri_validation_keep_original_rules() {
    let remote = association("reflector.example");

    for base in [
        "HTTPS://wallet.example/prefix?discard=1#discard",
        "https://[::1]:443/prefix",
        "https://wallet.example:/prefix",
    ] {
        let uri = association_uri(&remote, Some(base)).unwrap();
        assert_eq!(parse_association_uri(&uri).unwrap(), remote);
        assert!(!uri.contains("discard"));
    }

    for base in [
        "http://wallet.example",
        "//wallet.example",
        "https:/wallet.example",
        "https://wallet.example:65536",
        "https://[v1.a]/",
        "https://λ.example",
        "https://wallet.example/%GG",
        "https://wallet.example/\n",
    ] {
        assert!(
            association_uri(&remote, Some(base)).is_err(),
            "accepted {base}"
        );
    }
}
