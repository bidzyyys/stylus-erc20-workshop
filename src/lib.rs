//! Stylus ERC-20 Token Workshop.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use openzeppelin_stylus::access::ownable::{self, IOwnable, Ownable};
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
#[implements(IErc20<Error = erc20::Error>, IErc20Burnable<Error = erc20::Error>, IErc20Metadata, IErc165, IOwnable<Error = ownable::Error>)]
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
    type Error = ownable::Error;
    fn owner(&self) -> Address {
        self.ownable.owner()
    }

    fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), Self::Error> {
        self.ownable.transfer_ownership(new_owner)
    }

    fn renounce_ownership(&mut self) -> Result<(), Self::Error> {
        self.ownable.renounce_ownership()
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{uint, Address, U256};
    use motsu::prelude::*;

    use super::*;

    #[motsu::test]
    fn constructs(contract: Contract<Erc20Workshop>, alice: Address) {
        let name: String = "Stylus ERC-20 Workshop".to_string();
        let symbol: String = "MTK".to_string();

        contract
            .sender(alice)
            .constructor(name.clone(), symbol.clone(), alice)
            .motsu_expect("should construct");

        assert_eq!(name, contract.sender(alice).name());
        assert_eq!(symbol, contract.sender(alice).symbol());
        assert_eq!(U8::from(18), contract.sender(alice).decimals());
        assert_eq!(alice, contract.sender(alice).owner());
    }

    #[motsu::test]
    fn transfer_reverts_when_insufficient_balance(
        contract: Contract<Erc20Workshop>,
        alice: Address,
        bob: Address,
    ) {
        contract
            .sender(alice)
            .constructor(
                "Stylus ERC-20 Workshop".to_string(),
                "MTK".to_string(),
                alice,
            )
            .motsu_expect("should construct");

        let one = uint!(1_U256);

        // Initialize state for the test case:
        // Alice's & Bob's balance as `one`.
        contract
            .sender(alice)
            .mint(alice, one)
            .motsu_expect("should mint tokens");

        // Store initial balance & supply.
        let initial_alice_balance = contract.sender(alice).balance_of(alice);
        let initial_bob_balance = contract.sender(alice).balance_of(bob);

        // Transfer action should NOT work - `InsufficientBalance`.
        let err = contract
            .sender(alice)
            .transfer(bob, one + one)
            .motsu_unwrap_err();

        assert!(matches!(
            err,
            erc20::Error::InsufficientBalance(erc20::ERC20InsufficientBalance {
                sender,
                balance,
                needed,
            }) if sender == alice && balance == initial_alice_balance && needed == one + one,
        ));

        // Check proper state (before revert).
        assert_eq!(
            initial_alice_balance,
            contract.sender(alice).balance_of(alice)
        );
        assert_eq!(initial_bob_balance, contract.sender(alice).balance_of(bob));
    }

    #[motsu::test]
    #[should_panic = "should not exceed `U256::MAX` for `total_supply`"]
    fn mint_reverts_when_arithmetic_overflow(contract: Contract<Erc20Workshop>, alice: Address) {
        contract
            .sender(alice)
            .constructor(
                "Stylus ERC-20 Workshop".to_string(),
                "MTK".to_string(),
                alice,
            )
            .motsu_expect("should construct");

        let one = uint!(1_U256);
        assert_eq!(U256::ZERO, contract.sender(alice).balance_of(alice));
        assert_eq!(U256::ZERO, contract.sender(alice).total_supply());

        // Initialize state for the test case:
        // Alice's balance as `U256::MAX`.
        contract
            .sender(alice)
            .mint(alice, U256::MAX)
            .motsu_expect("should mint tokens");
        // Mint action should NOT work:
        // overflow on `total_supply`.
        let _result = contract.sender(alice).mint(alice, one);
    }

    #[motsu::test]
    fn transfers(contract: Contract<Erc20Workshop>, alice: Address, bob: Address) {
        contract
            .sender(alice)
            .constructor(
                "Stylus ERC-20 Workshop".to_string(),
                "MTK".to_string(),
                alice,
            )
            .motsu_expect("should construct");

        let one = uint!(1_U256);

        // Initialize state for the test case:
        //  Alice's & Bob's balance as `one`.
        contract
            .sender(alice)
            .mint(alice, one)
            .motsu_expect("should mint tokens");

        // Store initial balance & supply.
        let initial_alice_balance = contract.sender(alice).balance_of(alice);
        let initial_bob_balance = contract.sender(alice).balance_of(bob);

        // Transfer action should work.
        let result = contract.sender(alice).transfer(bob, one);
        assert!(result.is_ok());

        // Check updated balance & supply.
        assert_eq!(
            initial_alice_balance - one,
            contract.sender(alice).balance_of(alice)
        );
        assert_eq!(
            initial_bob_balance + one,
            contract.sender(alice).balance_of(bob)
        );

        contract.assert_emitted(&erc20::Transfer {
            from: alice,
            to: bob,
            value: one,
        });
    }
}
