//! Helper module to abstract over transactions
//!
//! TODO What is the significance of transactions in the MCD library? This module
//! is currently not part of the public API, should/must we change that? E.g. are
//! transactions executed in an atomic order?

use crate::mcd_bindings::{mcd_addr_st, mcd_tx_st, MCD_TX_AT_R, MCD_TX_AT_W, MCD_TX_OPT_DEFAULT};

pub enum Type {
    Read,
    Write,
}

impl Type {
    fn as_access_type(&self) -> u32 {
        match self {
            Type::Read => MCD_TX_AT_R as u32,
            Type::Write => MCD_TX_AT_W as u32,
        }
    }
}

/// Helper method to create a transaction
pub fn create_transaction(address: u64, transaction_type: Type, buffer: &mut [u8]) -> mcd_tx_st {
    mcd_tx_st {
        addr: mcd_addr_st {
            address,
            mem_space_id: 0,
            addr_space_id: 0,
            addr_space_type: 0,
        },
        access_type: transaction_type.as_access_type(),
        options: MCD_TX_OPT_DEFAULT as u32,
        access_width: 0,
        core_mode: 0,
        data: buffer.as_mut_ptr(),
        num_bytes: buffer.len() as u32,
        num_bytes_ok: 0,
    }
}
