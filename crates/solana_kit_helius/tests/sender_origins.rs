//! Every public Sender helper keeps submission and ping traffic on HTTPS.

use solana_kit_helius::sender::SenderRegion;

#[test]
fn every_region_uses_the_documented_auto_routed_https_endpoint() {
    for region in [
        SenderRegion::Default,
        SenderRegion::UsSlc,
        SenderRegion::UsEast,
        SenderRegion::EuWest,
        SenderRegion::EuCentral,
        SenderRegion::EuNorth,
        SenderRegion::ApSingapore,
        SenderRegion::ApTokyo,
    ] {
        assert_eq!(region.origin(), "https://sender.helius-rpc.com");
        assert_eq!(region.fast_url(), "https://sender.helius-rpc.com/fast");
        assert_eq!(region.ping_url(), "https://sender.helius-rpc.com/ping");
    }
}
