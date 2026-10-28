//! Distinct saved and backup keys make an accidental storage-source fallback observable.

use super::{LOCAL_ACCOUNT, fresh_wallet};
use crate::{Result, accessibility, require, validator::Validator};
use leptoskit_testing::native::{Locator, NativeTester, Platform};

enum Storage {
    Saved,
    Backup,
}

impl Storage {
    fn action(&self) -> &'static str {
        match self {
            Self::Saved => "Load saved wallet",
            Self::Backup => "Restore private copy",
        }
    }

    fn checkpoint(&self) -> &'static str {
        match self {
            Self::Saved => "Saved wallet loaded and signed after process relaunch",
            Self::Backup => "Distinct private copy restored and signed after process relaunch",
        }
    }
}

pub(crate) struct StoredWallet {
    address: String,
    storage: Storage,
}

pub(crate) struct Wallets {
    pub(crate) saved: StoredWallet,
    pub(crate) backup: StoredWallet,
}

/// The first funded key is copied privately; a different fresh key is saved.
/// Neither operation exports or reads a private key through the controller.
pub(super) fn save_other_wallet(
    tester: &mut NativeTester,
    platform: Platform,
    backup: String,
) -> Result<Wallets> {
    let saved = fresh_wallet(tester, platform)?;
    require(saved != backup, "saved and backup wallets must differ")?;
    tester.tap(&accessibility::action(platform, "Save wallet privately"))?;
    tester.expect_visible(&Locator::text("Wallet saved privately"))?;
    tester.checkpoint("A different native wallet was explicitly saved")?;
    tester.tap(&accessibility::action(platform, "Disconnect wallet"))?;
    tester.expect_visible(&accessibility::action(platform, "Create local wallet"))?;

    Ok(Wallets {
        saved: StoredWallet {
            address: saved,
            storage: Storage::Saved,
        },
        backup: StoredWallet {
            address: backup,
            storage: Storage::Backup,
        },
    })
}

impl StoredWallet {
    /// A new OS automation session relaunches the ordinary generated host with
    /// its existing private app data. Selection and signing still need real taps.
    pub(crate) fn verify(
        &self,
        tester: &mut NativeTester,
        node: &mut Validator,
        platform: Platform,
    ) -> Result<()> {
        node.alive()?;
        let balance = node.rpc.balance(&self.address)?;
        tester.expect_visible(&accessibility::action(platform, "Create local wallet"))?;
        tester.expect_absent(&Locator::accessibility_id(LOCAL_ACCOUNT))?;
        tester.tap(&accessibility::action(platform, self.storage.action()))?;
        tester.expect_visible(&accessibility::action(platform, "Review signature"))?;
        tester.checkpoint("Explicit wallet selection after process relaunch")?;
        require(
            accessibility::value(tester, LOCAL_ACCOUNT)? == self.address,
            "relaunched selection returned a different wallet",
        )?;

        tester.tap(&accessibility::action(platform, "Read balance"))?;
        tester.expect_visible(&Locator::text(format!("Balance: {balance} lamports")))?;
        tester.tap(&accessibility::action(platform, "Review signature"))?;
        tester.expect_visible(&Locator::text("Approve local wallet signature"))?;
        tester.tap(&accessibility::action(platform, "Approve signature"))?;
        tester.expect_visible(&Locator::text("Signature independently verified"))?;
        tester.checkpoint(self.storage.checkpoint())?;
        require(
            node.rpc.balance(&self.address)? == balance,
            "wallet recovery changed funds",
        )?;

        tester.tap(&accessibility::action(platform, "Disconnect wallet"))?;
        tester.expect_visible(&accessibility::action(platform, "Create local wallet"))?;
        Ok(())
    }
}
