use solana_kit_core::{AccountMeta, AccountRole, Address, codecs::Decoder};
use solana_kit_loader::v3::*;
#[path = "../../../verification/stake-loader-portable/src/contracts.rs"]
mod contracts;
use contracts::*;
const ADDRESS_SIZE: usize = 32;
const ACCOUNT_1: Address = Address::from_bytes([1; ADDRESS_SIZE]);
const ACCOUNT_2: Address = Address::from_bytes([2; ADDRESS_SIZE]);
const ACCOUNT_3: Address = Address::from_bytes([3; ADDRESS_SIZE]);
const ACCOUNT_4: Address = Address::from_bytes([4; ADDRESS_SIZE]);
const ACCOUNT_5: Address = Address::from_bytes([5; ADDRESS_SIZE]);
const ACCOUNT_6: Address = Address::from_bytes([6; ADDRESS_SIZE]);
const ACCOUNT_7: Address = Address::from_bytes([7; ADDRESS_SIZE]);
const ACCOUNT_8: Address = Address::from_bytes([8; ADDRESS_SIZE]);
#[path = "v3/initializebuffer.rs"]
mod initializebuffer;
#[cfg_attr(test, test)]
pub fn initializebuffer_metadata_contract() {
    initializebuffer::run();
}
#[path = "v3/write.rs"]
mod write;
#[cfg_attr(test, test)]
pub fn write_metadata_contract() {
    write::run();
}
#[path = "v3/deploywithmaxdatalen.rs"]
mod deploywithmaxdatalen;
#[cfg_attr(test, test)]
pub fn deploywithmaxdatalen_metadata_contract() {
    deploywithmaxdatalen::run();
}
#[path = "v3/upgrade.rs"]
mod upgrade;
#[cfg_attr(test, test)]
pub fn upgrade_metadata_contract() {
    upgrade::run();
}
#[path = "v3/setauthority.rs"]
mod setauthority;
#[cfg_attr(test, test)]
pub fn setauthority_metadata_contract() {
    setauthority::run();
}
#[path = "v3/close.rs"]
mod close;
#[cfg_attr(test, test)]
pub fn close_metadata_contract() {
    close::run();
}
#[path = "v3/extendprogram.rs"]
mod extendprogram;
#[cfg_attr(test, test)]
pub fn extendprogram_metadata_contract() {
    extendprogram::run();
}
#[path = "v3/setauthoritychecked.rs"]
mod setauthoritychecked;
#[cfg_attr(test, test)]
pub fn setauthoritychecked_metadata_contract() {
    setauthoritychecked::run();
}
