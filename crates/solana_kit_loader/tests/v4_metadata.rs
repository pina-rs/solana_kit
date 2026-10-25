use solana_kit_core::{AccountMeta, AccountRole, Address, codecs::Decoder};
use solana_kit_loader::v4::*;
#[path = "../../../verification/stake-loader-portable/src/contracts.rs"]
mod contracts;
use contracts::*;
const ADDRESS_SIZE: usize = 32;
const ACCOUNT_1: Address = Address::from_bytes([1; ADDRESS_SIZE]);
const ACCOUNT_2: Address = Address::from_bytes([2; ADDRESS_SIZE]);
const ACCOUNT_3: Address = Address::from_bytes([3; ADDRESS_SIZE]);
#[path = "v4/write.rs"]
mod write;
#[cfg_attr(test, test)]
pub fn write_metadata_contract() {
    write::run();
}
#[path = "v4/truncate.rs"]
mod truncate;
#[cfg_attr(test, test)]
pub fn truncate_metadata_contract() {
    truncate::run();
}
#[path = "v4/deploy.rs"]
mod deploy;
#[cfg_attr(test, test)]
pub fn deploy_metadata_contract() {
    deploy::run();
}
#[path = "v4/retract.rs"]
mod retract;
#[cfg_attr(test, test)]
pub fn retract_metadata_contract() {
    retract::run();
}
#[path = "v4/transferauthority.rs"]
mod transferauthority;
#[cfg_attr(test, test)]
pub fn transferauthority_metadata_contract() {
    transferauthority::run();
}
#[path = "v4/finalize.rs"]
mod finalize;
#[cfg_attr(test, test)]
pub fn finalize_metadata_contract() {
    finalize::run();
}
