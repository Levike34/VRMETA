#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod metascore {

use ink_storage::traits::SpreadAllocate;
use ink_storage::Mapping;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    /// Returned if caller is not the owner.
    NonOwner,
}

#[ink(event)]
pub struct MatchStarted {
    #[ink(topic)]
    caller: Option<AccountId>,
}

#[ink(event)]
pub struct MatchFinished {
    #[ink(topic)]
    caller: Option<AccountId>,
    #[ink(topic)]
    score: Option<u32>
}

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Metascore {
        /// Stores a single `bool` value on the storage.
        score: Mapping<AccountId, u32>,
        /// Match Started
        in_match: Mapping<AccountId, bool>,
    }
    
    pub type Result<T> = core::result::Result<T, Error>;

    impl Metascore {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            // Even though we're not explicitly initializing the `Mapping`,
            // we still need to call this
            ink_lang::utils::initialize_contract(Self::new_init)
            
        }

         /// Default initializes the contract.
    fn new_init(&mut self) {
        let caller = Self::env().caller();
        self.score.insert(caller, &0);
        self.in_match.insert(caller , &false);
    }
        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn start_match(&mut self) {
            let caller: AccountId = self.env().caller();
            self.score.insert(caller, &0);
            self.in_match.insert(caller , &true);
            self.env().emit_event(MatchStarted {
                caller: Some(caller)
            });
        }

        #[ink(message)]
        pub fn finish_match(&mut self, points: u32) {
            let caller: AccountId = self.env().caller();
            self.score.insert(caller, &points);
            self.in_match.insert(caller , &false);

            ink_env::debug_println!("requested value: {}", points);
            ink_env::debug_println!("contract balance: {}", self.env().balance());

            assert!(u128::from(points) <= self.env().balance(), "insufficient funds!");

            if self.env().transfer(self.env().caller(), points.into()).is_err() {
                panic!(
                    "Hold up."
                )
            }

            self.env().emit_event(MatchFinished {
                caller: Some(caller),
                score: Some(points)
            });
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_score(&self) -> u32 {
            let caller: AccountId = self.env().caller();
            let score = self.score.get(&caller).unwrap();
            score
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let metasign = Metascore::default();
            assert_eq!(metasign.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut metasign = Metascore::new(false);
            assert_eq!(metasign.get(), false);
            metasign.flip();
            assert_eq!(metasign.get(), true);
        }
    }
}
