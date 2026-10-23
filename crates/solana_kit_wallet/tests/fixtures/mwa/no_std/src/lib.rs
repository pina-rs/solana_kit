//! Ordinary no_std consumer: std anywhere in the closure duplicates panic_impl.
#![no_std]

use solana_kit_wallet::WalletFeatures;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

/// Keeps an ordinary public wallet type in the consumer.
pub fn wallet_size() -> usize {
    core::mem::size_of::<WalletFeatures>()
}

/// Keeps a public protocol type in the feature-enabled consumer.
#[cfg(feature = "mwa")]
pub fn association_size() -> usize {
    core::mem::size_of::<solana_kit_wallet::mwa::Association>()
}
