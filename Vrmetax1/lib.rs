#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod vrmetax1 {

use ink_storage::traits::SpreadAllocate;
use ink_storage::Mapping;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    /// Returned if caller is not the owner.
    NonOwner,
}

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
#[ink(storage)]
#[derive(SpreadAllocate)]
   pub struct Vrmetax1 {
    /// Points from the game.
    pub points: Mapping<AccountId, u32>,
    /// Master wallet who receives funds from game.
    pub master_address: AccountId,
    /// Items in the game.  Can be customized according to the item itself.
    pub ammo: Mapping<AccountId, Balance>,
    /// Items.
    pub missiles: Mapping<AccountId, Balance>,
    /// Gun Rights
    pub gun_rights: Mapping<AccountId, bool>,
    /// NFT Skins
    pub nft_skins: Mapping<AccountId, bool>
}

    impl Vrmetax1 {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            // Even though we're not explicitly initializing the `Mapping`,
            // we still need to call this
            ink_lang::utils::initialize_contract(Self::new_init) 
        }

         /// Default initializes the contract.
    fn new_init(&mut self) {
        let caller = Self::env().caller();
        self.master_address = caller;
        self.ammo.insert(caller, &0);
        self.missiles.insert(caller, &0);
        self.gun_rights.insert(caller, &false);
        self.nft_skins.insert(caller, &false);
    }

        #[ink(message, payable)]
        pub fn buy_ammo(&mut self) {
            let caller: AccountId = self.env().caller();
            let price: Balance = 1_000_000_000;

            //assert!(sent_amount >= 1_000000000);

            let amount: Balance = self.env().transferred_value();
            assert!(amount > price, "1 coin or naught");

            let bullets: Balance = amount / 1_000_000_000;

            if self.env().transfer(self.master_address, amount).is_err() {
                panic!(
                    "Funding problem."
                )
            }

            let old_amount = self.ammo.get(&caller).unwrap();

            self.ammo.insert(caller, &(&bullets + &old_amount));
        }

        #[ink(message, payable)]
        pub fn buy_missiles(&mut self) {
            let caller: AccountId = self.env().caller();
            let price: Balance = 10_000_000_000;

            let amount = self.env().transferred_value();

            assert!(amount > price, "10 coins or naught");
            let missiles = amount / 10_000_000_000;

            if self.env().transfer(self.master_address, amount).is_err() {
                panic!(
                    "Funding problem."
                )
            }
            let old_amount = self.missiles.get(&caller).unwrap();

            self.missiles.insert(caller, &(&missiles + &old_amount));
        }

        #[ink(message, payable)]
        pub fn buy_gun_rights(&mut self) {
            let caller: AccountId = self.env().caller();
            let price: Balance = 50_000_000_000;

            let amount = self.env().transferred_value();
            assert!(amount > price, "50 coins or naught");

            if self.env().transfer(self.master_address, amount).is_err() {
                panic!(
                    "Funding problem."
                )
            }

            self.gun_rights.insert(caller, &true);
        }

        #[ink(message, payable)]
        pub fn buy_nft_skin(&mut self) {
            let caller: AccountId = self.env().caller();
            let price: Balance = 200_000_000_000;

            let amount = self.env().transferred_value();
            assert!(amount > price, "200 coins or naught");

            if self.env().transfer(self.master_address, amount).is_err() {
                panic!(
                    "Funding problem."
                )
            }

            self.nft_skins.insert(caller, &true);
        }

        #[ink(message)]
        pub fn shoot_ammo(&mut self, amount_bullets: Balance) {
            let caller: AccountId = self.env().caller();
            let ammo_before = self.ammo.get(&caller).unwrap();
            let amount_left = ammo_before - amount_bullets;
            self.ammo.insert(caller, &amount_left)
        }

        /// Return ammo
        #[ink(message)]
        pub fn get_ammo(&self) -> Balance {
            let caller: AccountId = self.env().caller();
            self.ammo.get(&caller).unwrap()
        }

         /// Return ammo
         #[ink(message)]
         pub fn get_missiles(&self) -> Balance {
             let caller: AccountId = self.env().caller();
             self.missiles.get(&caller).unwrap()
         }

         #[ink(message)]
         pub fn get_gun_rights(&self) -> bool {
             let caller: AccountId = self.env().caller();
             self.gun_rights.get(&caller).unwrap()
         }

         #[ink(message)]
         pub fn get_owns_nft_skin(&self) -> bool {
             let caller: AccountId = self.env().caller();
             self.gun_rights.get(&caller).unwrap()
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
            let erc1155 = Erc1155::default();
            assert_eq!(erc1155.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut erc1155 = Erc1155::new(false);
            assert_eq!(erc1155.get(), false);
            erc1155.flip();
            assert_eq!(erc1155.get(), true);
        }
    }
}


