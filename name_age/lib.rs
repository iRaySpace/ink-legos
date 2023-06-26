#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod name_age {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct NameAge {
        name: String,
        age: u32,
    }

    impl NameAge {
        #[ink(constructor)]
        pub fn new(init_name: String, init_age: u32) -> Self {
            Self { name: init_name, age: init_age }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(), Default::default())
        }

        #[ink(message)]
        pub fn set_name(&mut self, new_name: String) {
            self.name = new_name;
        }

        #[ink(message)]
        pub fn set_age(&mut self, new_age: u32) {
            self.age = new_age;
        }

        #[ink(message)]
        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        #[ink(message)]
        pub fn get_age(&self) -> u32 {
            self.age
        }
    }
}
