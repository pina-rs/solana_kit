//! Actual interpreter and Bridge identities independently constrain socket work.
use super::*;
use solana_kit_leptoskit::HostError;

#[tokio::test]
async fn foreign_driver_rejects_completion_without_entering_guest() {
    let (endpoint, peer, _signal) = fixture(false).await;
    let mut host = host();
    let mut first = driver(&mut host);
    let plugin = install(&mut host, &mut first, endpoint);
    submit(&mut host, plugin, 1, Vec::new());
    route(&mut host, &mut first);
    let completion = next(&mut first).await;
    let mut other = driver(&mut host);

    assert!(matches!(
        other.deliver(&mut host, completion),
        Err(HostError::Owner)
    ));
    assert!(host.plugins().event().is_none());
    first.shutdown().await.unwrap();
    other.shutdown().await.unwrap();
    joined(peer).await;
}

#[tokio::test]
async fn bridge_transplant_and_replacement_reject_both_axes() {
    let mut first = host();
    let mut second = host();
    let mut driver = driver(&mut first);
    *second.plugins() = std::mem::take(first.plugins());
    assert!(matches!(
        driver.route(&mut second, Default::default()),
        Err(HostError::Owner)
    ));
    assert!(matches!(
        driver.route(&mut first, Default::default()),
        Err(HostError::Owner)
    ));
    driver.shutdown().await.unwrap();
}

#[tokio::test]
async fn moving_actual_host_preserves_socket_identity_and_drop_aborts() {
    let (endpoint, peer, _signal) = fixture(false).await;
    let mut host = host();
    let mut driver = driver(&mut host);
    let plugin = install(&mut host, &mut driver, endpoint);
    open(&mut host, &mut driver, plugin).await;
    let mut host = Box::new(host);
    driver.route(&mut host, Default::default()).unwrap();
    drop(driver);

    joined(peer).await;
}
