use solana_kit_core::{AccountMeta, AccountRole, Address, codecs::Decoder};
use solana_kit_stake::*;
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
#[path = "stake/initialize.rs"]
mod initialize;
#[cfg_attr(test, test)]
pub fn initialize_metadata_contract() {
    initialize::run();
}
#[path = "stake/authorize.rs"]
mod authorize;
#[cfg_attr(test, test)]
pub fn authorize_metadata_contract() {
    authorize::run();
}
#[path = "stake/delegatestake.rs"]
mod delegatestake;
#[cfg_attr(test, test)]
pub fn delegatestake_metadata_contract() {
    delegatestake::run();
}
#[path = "stake/split.rs"]
mod split;
#[cfg_attr(test, test)]
pub fn split_metadata_contract() {
    split::run();
}
#[path = "stake/withdraw.rs"]
mod withdraw;
#[cfg_attr(test, test)]
pub fn withdraw_metadata_contract() {
    withdraw::run();
}
#[path = "stake/deactivate.rs"]
mod deactivate;
#[cfg_attr(test, test)]
pub fn deactivate_metadata_contract() {
    deactivate::run();
}
#[path = "stake/setlockup.rs"]
mod setlockup;
#[cfg_attr(test, test)]
pub fn setlockup_metadata_contract() {
    setlockup::run();
}
#[path = "stake/merge.rs"]
mod merge;
#[cfg_attr(test, test)]
pub fn merge_metadata_contract() {
    merge::run();
}
#[path = "stake/authorizewithseed.rs"]
mod authorizewithseed;
#[cfg_attr(test, test)]
pub fn authorizewithseed_metadata_contract() {
    authorizewithseed::run();
}
#[path = "stake/initializechecked.rs"]
mod initializechecked;
#[cfg_attr(test, test)]
pub fn initializechecked_metadata_contract() {
    initializechecked::run();
}
#[path = "stake/authorizechecked.rs"]
mod authorizechecked;
#[cfg_attr(test, test)]
pub fn authorizechecked_metadata_contract() {
    authorizechecked::run();
}
#[path = "stake/authorizecheckedwithseed.rs"]
mod authorizecheckedwithseed;
#[cfg_attr(test, test)]
pub fn authorizecheckedwithseed_metadata_contract() {
    authorizecheckedwithseed::run();
}
#[path = "stake/setlockupchecked.rs"]
mod setlockupchecked;
#[cfg_attr(test, test)]
pub fn setlockupchecked_metadata_contract() {
    setlockupchecked::run();
}
#[path = "stake/getminimumdelegation.rs"]
mod getminimumdelegation;
#[cfg_attr(test, test)]
pub fn getminimumdelegation_metadata_contract() {
    getminimumdelegation::run();
}
#[path = "stake/deactivatedelinquent.rs"]
mod deactivatedelinquent;
#[cfg_attr(test, test)]
pub fn deactivatedelinquent_metadata_contract() {
    deactivatedelinquent::run();
}
#[path = "stake/movestake.rs"]
mod movestake;
#[cfg_attr(test, test)]
pub fn movestake_metadata_contract() {
    movestake::run();
}
#[path = "stake/movelamports.rs"]
mod movelamports;
#[cfg_attr(test, test)]
pub fn movelamports_metadata_contract() {
    movelamports::run();
}
