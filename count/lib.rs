#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod count {
    #[ink(storage)]
    pub struct Count {
        value: u32,
    }

    impl Count {
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn increment(&mut self) {
            self.value = self.value + 1;
        }

        #[ink(message)]
        pub fn decrement(&mut self) {
            if self.value > 0 {
                self.value = self.value - 1;
            }
        }

        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }
    }
}
