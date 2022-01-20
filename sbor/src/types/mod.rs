mod address;
mod bid;
mod big_decimal;
mod decimal;
mod h256;
mod mid;
mod rid;
mod vid;

pub const SCRYPTO_TYPE_DECIMAL: u8 = 0x80;
pub const SCRYPTO_TYPE_BIG_DECIMAL: u8 = 0x81;
pub const SCRYPTO_TYPE_ADDRESS: u8 = 0x82;
pub const SCRYPTO_TYPE_H256: u8 = 0x83;
pub const SCRYPTO_TYPE_BID: u8 = 0x84;
pub const SCRYPTO_TYPE_RID: u8 = 0x85;
pub const SCRYPTO_TYPE_MID: u8 = 0x86;
pub const SCRYPTO_TYPE_VID: u8 = 0x87;

// TODO: rename
pub const SCRYPTO_NAME_DECIMAL: &str = "scrypto::types::Decimal";
pub const SCRYPTO_NAME_BIG_DECIMAL: &str = "scrypto::types::BigDecimal";
pub const SCRYPTO_NAME_ADDRESS: &str = "scrypto::types::Address";
pub const SCRYPTO_NAME_H256: &str = "scrypto::types::H256";
pub const SCRYPTO_NAME_BID: &str = "scrypto::types::Bid";
pub const SCRYPTO_NAME_RID: &str = "scrypto::types::Rid";
pub const SCRYPTO_NAME_MID: &str = "scrypto::types::Mid";
pub const SCRYPTO_NAME_VID: &str = "scrypto::types::Vid";

pub use address::{
    Address, ParseAddressError, ACCOUNT_PACKAGE, RADIX_TOKEN, SYSTEM_COMPONENT, SYSTEM_PACKAGE,
};
pub use bid::{Bid, ParseBidError};
pub use big_decimal::{BigDecimal, ParseBigDecimalError};
pub use decimal::{Decimal, ParseDecimalError};
pub use h256::{ParseH256Error, H256};
pub use mid::{Mid, ParseMidError};
pub use rid::{ParseRidError, Rid};
pub use vid::{ParseVidError, Vid};

use crate::rust::vec::Vec;

fn copy_u8_array<const N: usize>(slice: &[u8]) -> [u8; N] {
    if slice.len() == N {
        let mut bytes = [0u8; N];
        bytes.copy_from_slice(&slice[0..N]);
        bytes
    } else {
        panic!("Invalid length");
    }
}

fn combine(ty: u8, bytes: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(1 + bytes.len());
    v.push(ty);
    v.extend(bytes);
    v
}
