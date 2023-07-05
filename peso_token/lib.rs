#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod peso_token {
    use openbrush::{
        contracts::psp22::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct PesoToken {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl PSP22 for PesoToken {}

    impl PesoToken {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            instance._mint_to(Self::env().caller(), total_supply).expect("should mint");
            instance
        }
    }
}