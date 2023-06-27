#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod conferences_attended {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Lazy;
    use scale::{Decode, Encode};

    #[derive(Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Attendance {
        name: String,
    }

    #[derive(Default)]
    #[ink(storage)]
    pub struct ConferencesAttended {
        attendances: Lazy<Vec<Attendance>>,
    }

    impl ConferencesAttended {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn get_attendances(&self) -> Vec<Attendance> {
            self.attendances.get_or_default()
        }

        #[ink(message)]
        pub fn add_attendance(&mut self, value: Attendance) {
            let mut attendances = self.attendances.get_or_default();
            attendances.push(value);
            self.attendances.set(&attendances);
        }
    }
}