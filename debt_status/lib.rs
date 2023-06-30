#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod debt_status {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct DebtStatus {
        debts: Mapping<AccountId, String>,
    }

    impl DebtStatus {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { debts: Mapping::default() }
        }

        #[ink(message)]
        pub fn paid(&mut self) {
            let caller = self.env().caller();
            self.debts.insert(caller, &String::from("Paid"));
        }

        #[ink(message)]
        pub fn unpaid(&mut self) {
            let caller = self.env().caller();
            self.debts.insert(caller, &String::from("Unpaid"));
        }

        #[ink(message)]
        pub fn get(&self) -> String {
            let caller = self.env().caller();
            let status = self.debts.get(caller).unwrap_or(String::from(""));
            status.clone()
        }
    }
}
