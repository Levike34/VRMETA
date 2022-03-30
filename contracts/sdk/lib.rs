#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;



#[ink::contract]
mod sdk {

use ink_storage::traits::SpreadAllocate;
use ink_storage::Mapping;
pub use crate::stripe;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    /// Returned if caller is not the owner.
    NonOwner,
    /// Not enough money.
    PaymentDeclined
}

#[ink(event)]
pub struct Bought {
    #[ink(topic)]
    caller: Option<AccountId>,
    #[ink(topic)]
    amount: Option<u128>
}

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Sdk {
        /// Amount Bought
        bought: Mapping<AccountId, Balance>,
        /// Owner
        owner: AccountId
    }
    
    pub type Result<T> = core::result::Result<T, Error>;

    impl Sdk {
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
        self.bought.insert(caller , &0);
    }
      
    #[ink(message)]
    pub fn buy_vrmeta(&mut self) {
        let caller: AccountId = self.env().caller();

        /* Creating a Stripe Charge */
        let client = stripe::Client::new("sk_test_51KXzTBLr0bYuQcweg3FwiY83G24x0fT7nl6JfjFd97rJVVLdrY3fb1s7Rg8LQq8M75LlfZ4A9drvjuiQWhBr34Wa003Evod6MC");
        let token = "tok_ID_FROM_CHECKOUT".parse().unwrap();
        let mut params = stripe::CreateCharge::new();
        // NOTE: Stripe represents currency in the lowest denominations (e.g. cents)
        params.amount = Some(1095); // e.g. $10.95
        params.source = Some(stripe::ChargeSourceParams::Token(token));
        let amount = 1095u128;

        // Example: Override currency to be in Canadian Dollars
        params.currency = Some(stripe::Currency::USD);
        let charge = stripe::Charge::create(&client, params).unwrap();
        self.env().emit_event(Bought {
            caller: Some(caller),
            amount: Some(amount)
        });
        ink_env::debug_println!("{:?}", charge); // =>  Charge { id: "ch_12345", amount: 1095, .. }
        self.bought.insert(caller, &amount);
        }

  

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_bought(&self) -> u128 {
            let caller: AccountId = self.env().caller();
            let bought = self.bought.get(&caller).unwrap();
            bought
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


