#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;


#[ink::contract]
mod timestake {

use ink_storage::traits::SpreadAllocate;
use ink_storage::Mapping;

#[ink(event)]
pub struct Connected {
    caller: AccountId,
    timestamp: u64,
}

#[ink(event)]
pub struct Disconnected {
    caller: AccountId,
    timestamp: u64,
    reward_to_pay: u64
}

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Timestake {
        /// Checks if the user is Online
        is_connected:  Mapping<AccountId, bool>,
        /// Store start time
        start_time: Mapping<AccountId, u64>,
        //  Store end time
        end_time:  Mapping<AccountId, u64>,
        //  Reward accumulated
        reward:  Mapping<AccountId, Balance>,
        // Coins rewarded per hour
        reward_rate_per_hour: u64,
        // Owner
        owner: AccountId
        
    }

    impl Timestake {

        #[ink(constructor, payable)]
        pub fn new() -> Self {
            // Even though we're not explicitly initializing the `Mapping`,
            // we still need to call this
            ink_lang::utils::initialize_contract(Self::new_init)
            
        }

         /// Default initializes the contract.
    fn new_init(&mut self) {
        let caller = Self::env().caller();
        self.owner = caller;
        self.is_connected.insert(caller, &false);
        self.start_time.insert(caller , &0);
        self.end_time.insert(caller, &0);
        self.reward.insert(caller, &0);
        self.reward_rate_per_hour = 1_000_000_000;
    }

      
        #[ink(message)]
        pub fn connect(&mut self) {
            let caller: AccountId = self.env().caller();
            let timestamp = self.env().block_timestamp();
            self.is_connected.insert(caller, &true);
            self.start_time.insert(caller, &timestamp);
            self.env().emit_event(Connected {
                caller,
                timestamp
            });
        }

        #[ink(message, payable)]
        pub fn disconnect(&mut self) {
            let caller: AccountId = self.env().caller();
            let start_time = self.start_time.get(&caller).unwrap();
            let timestamp = self.env().block_timestamp();
            let my_time_played: u64 = timestamp - start_time;
            self.is_connected.insert(caller, &false);
            self.start_time.insert(caller, &0);
            let reward_to_pay: u64 = (my_time_played / 3600) * self.reward_rate_per_hour;
            self.reward.insert(caller, &0);
            
            ink_env::debug_println!("requested value: {}", reward_to_pay);
            ink_env::debug_println!("contract balance: {}", self.env().balance());

            assert!(u128::from(reward_to_pay) <= self.env().balance(), "insufficient funds!");

            if self.env().transfer(self.env().caller(), reward_to_pay.into()).is_err() {
                panic!(
                    "Some hol' up bro."
                )
            }
            self.env().emit_event(Disconnected {
                caller,
                timestamp,
                reward_to_pay
            });
        }

        /// Simply returns the current value of our reward per hour.
        #[ink(message)]
        pub fn get_reward_hourly(&self) -> u64 {
            self.reward_rate_per_hour.try_into().unwrap()
        }

        #[ink(message)]
        pub fn set_reward_hourly(&mut self, tokens_per_hour: u64) {
            self.reward_rate_per_hour = tokens_per_hour * 1_000_000_000_000
        }



        #[ink(message)]
        pub fn get_total_balance(&self) -> u128 {
            self.env().balance()
        }

      

        #[ink(message)]
        pub fn get_start_time(&self) -> u64 {
            let caller: AccountId = self.env().caller();
            let start_time = self.start_time.get(&caller).unwrap();
            start_time
        }

        /// Calculate time played in seconds
        #[ink(message)]
        pub fn get_time_played(&self) -> u64  {
            let caller: AccountId = self.env().caller();
            let start_time = self.start_time.get(&caller).unwrap();
            if start_time < 1 {
                0
            } else {
                let timestamp = self.env().block_timestamp();
                let amount = timestamp - start_time;
                amount
            }
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
            let timestake = Timestake::default();
            assert_eq!(timestake.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut timestake = Timestake::new(false);
            assert_eq!(timestake.get(), false);
            timestake.flip();
            assert_eq!(timestake.get(), true);
        }
    }
}
