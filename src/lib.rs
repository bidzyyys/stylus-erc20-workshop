//! Stylus ERC-20 Token Workshop.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use openzeppelin_stylus::token::erc20::{self, Erc20, IErc20};
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    prelude::*,
};

// Define some persistent storage.
// `Erc20Workshop` will be the entrypoint.
#[entrypoint]
#[storage]
struct Erc20Workshop {
    erc20: Erc20,
}

/// Declare that `Erc20Workshop` is a contract with the following external methods.
#[public]
#[implements(IErc20<Error = erc20::Error>)]
impl Erc20Workshop {}

#[public]
impl IErc20 for Erc20Workshop {
    type Error = erc20::Error;

    fn total_supply(&self) -> U256 {
        self.erc20.total_supply()
    }

    fn balance_of(&self, account: Address) -> U256 {
        self.erc20.balance_of(account)
    }

    fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Self::Error> {
        self.erc20.transfer(to, value)
    }

    fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.erc20.allowance(owner, spender)
    }

    fn approve(&mut self, spender: Address, value: U256) -> Result<bool, Self::Error> {
        self.erc20.approve(spender, value)
    }

    fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Self::Error> {
        self.erc20.transfer_from(from, to, value)
    }
}
