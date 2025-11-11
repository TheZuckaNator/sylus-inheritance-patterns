#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

// Modules
mod contract_a;
mod contract_b;

// Import contract from module
use contract_a::ContractA;
use contract_b::ContractB; //  1. Import ContractB from contract_b module

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

sol_storage! {
    #[entrypoint]
    pub struct Foo {
        ContractA contract_a;
        ContractB contract_b;
    }
}

// Define trait for ContractA
pub trait IContractA {
    fn ret_num_a(&self) -> U256;
}

// Implement trait `IContractA`
#[public] //  2. Make the impl block public
impl IContractA for Foo {
    fn ret_num_a(&self) -> U256 {
        self.contract_a.ret_num()
    }
}

// Define trait for ContractB
pub trait IContractB {
    fn ret_num_b(&self) -> U256; 
}

// Implement trait `IContractB`
#[public]
impl IContractB for Foo {
    fn ret_num_b(&self) -> U256 {
        self.contract_b.ret_numb() //  3. Call ret_numb() from contract_b
    }
}

/// Declare that `Foo` is a contract with the following external methods.
#[public]
#[implements(IContractA, IContractB)] //  4. Declare trait implementations
impl Foo {
    pub fn foofoo(&self) -> U256 {
        self.ret_num_a()
    }
}