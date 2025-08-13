//! Stylus ERC-20 Token Workshop.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use openzeppelin_stylus::access::ownable::{IOwnable, Ownable};
use openzeppelin_stylus::token::erc20::{
    self,
    extensions::{Erc20Metadata, IErc20Burnable, IErc20Metadata},
    Erc20, IErc20,
};
use openzeppelin_stylus::utils::introspection::erc165::IErc165;
use stylus_sdk::{
    alloy_primitives::{aliases::B32, Address, U256, U8},
    prelude::*,
};

// Define some persistent storage.
// `Erc20Workshop` will be the entrypoint.
#[entrypoint]
#[storage]
struct Erc20Workshop {
    erc20: Erc20,
    metadata: Erc20Metadata,
    ownable: Ownable,
}

/// Declare that `Erc20Workshop` is a contract with the following external methods.
#[public]
#[implements(IErc20<Error = erc20::Error>, IErc20Burnable<Error = erc20::Error>, IErc20Metadata, IErc165, IOwnable)]
impl Erc20Workshop {
    #[constructor]
    pub fn constructor(
        &mut self,
        name: String,
        symbol: String,
        owner: Address,
    ) -> Result<(), Vec<u8>> {
        self.ownable.constructor(owner)?;
        self.metadata.constructor(name, symbol);
        Ok(())
    }

    pub fn mint(&mut self, to: Address, value: U256) -> Result<(), Vec<u8>> {
        self.ownable.only_owner()?;
        self.erc20._mint(to, value)?;
        Ok(())
    }
}

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

#[public]
impl IErc20Burnable for Erc20Workshop {
    type Error = erc20::Error;

    fn burn(&mut self, value: U256) -> Result<(), Self::Error> {
        self.erc20.burn(value)
    }

    fn burn_from(&mut self, account: Address, value: U256) -> Result<(), Self::Error> {
        self.erc20.burn_from(account, value)
    }
}

#[public]
impl IErc20Metadata for Erc20Workshop {
    fn name(&self) -> String {
        self.metadata.name()
    }

    fn symbol(&self) -> String {
        self.metadata.symbol()
    }

    fn decimals(&self) -> U8 {
        self.metadata.decimals()
    }
}

#[public]
impl IErc165 for Erc20Workshop {
    fn supports_interface(&self, interface_id: B32) -> bool {
        self.erc20.supports_interface(interface_id)
            || self.metadata.supports_interface(interface_id)
    }
}

#[public]
impl IOwnable for Erc20Workshop {
    fn owner(&self) -> Address {
        self.ownable.owner()
    }

    fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), Vec<u8>> {
        Ok(self.ownable.transfer_ownership(new_owner)?)
    }

    fn renounce_ownership(&mut self) -> Result<(), Vec<u8>> {
        Ok(self.ownable.renounce_ownership()?)
    }
}
