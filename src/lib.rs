#![cfg_attr(not(feature = "export-abi"), no_std)]
extern crate alloc;

mod erc20;

use crate::erc20::{ERC20, ERC20Params};
use alloc::{vec::Vec};
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};
use alloy_primitives::{B256};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct SampleParams;

/// Immutable definitions
impl ERC20Params for SampleParams {
    const NAME: &'static str = "MyToken";
    const SYMBOL: &'static str = "MTK";
    const DECIMALS: u8 = 18;
    const INITIAL_CHAIN_ID: u64 = 1;
    const INITIAL_DOMAIN_SEPARATOR: B256 = B256::ZERO;
}

// The contract
sol_storage! {
    #[entrypoint] // Makes MyToken the entrypoint
    pub struct MyToken {
        #[borrow] // Allows erc20 to access MyToken's storage and make calls
        ERC20<SampleParams> erc20;
        uint256 total_supply;
    }
}

#[external]
#[inherit(ERC20<SampleParams>)]
impl MyToken {
    pub fn mint(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        self.erc20.mint(msg::sender(), amount);

        Ok(())
    }
}