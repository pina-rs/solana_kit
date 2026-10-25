//! Fixed SDK bridge methods; these do not extend portable MWA vocabulary.
pub const BEGIN_MOBILE: u32 = 0;
pub const CONNECT_MOBILE: u32 = 1;
pub const AUTHORIZE: u32 = 2;
pub const SIGN_TRANSACTIONS: u32 = 3;
pub const SIGN_MESSAGES: u32 = 4;
pub const SEND_TRANSACTIONS: u32 = 5;
pub const DEAUTHORIZE: u32 = 6;
pub const CLOSE: u32 = 7;

pub const CREATE_LOCAL: u32 = 8;
pub const LOAD_LOCAL: u32 = 9;
pub const SIGN_LOCAL: u32 = 10;
pub const SAVE_LOCAL: u32 = 11;
pub const METHODS: u32 = 12;
pub const OPENED: u32 = 1;
pub const CALL_BYTES: usize = 8;
pub const POLL_BUDGET: usize = 32;

pub fn starting(method: u32) -> bool {
    matches!(method, BEGIN_MOBILE | CREATE_LOCAL | LOAD_LOCAL)
}
