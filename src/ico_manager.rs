#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const EGLD_NUM_DECIMALS: usize = 18;

/// Manage ICO of a new ESDT
#[elrond_wasm::contract]
pub trait IcoManeger {
    #[init]
    fn init(&self) {}
}
