#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod lol {
    #[ink(storage)]
    pub struct Lol {}

    impl Lol {
        #[ink(constructor)]
        pub fn default() -> Self {
            Lol {}
        }

        #[cfg(feature = "upgrade")]
        #[ink(message)]
        pub fn call(&self) {}
    }
}
