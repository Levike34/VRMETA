#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

///add class system, ability system, stats, level system, consumables

#[ink::contract]
mod vrmetaxrpg {

use ink_storage::traits::SpreadAllocate;
use ink_storage::Mapping;

pub type Name = AccountId;
pub type Class = u8;
pub type Lvl = u32;
pub type Character = (Name, Class, Lvl);

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
   pub struct VrmetaxRPG {
    /// Points from the game as experience.
    pub points: Mapping<AccountId, u32>,
    /// Master wallet who receives funds from game.
    pub master_address: AccountId,
    /// Items in the game.  Can be customized according to the item itself.
    pub consumables: Mapping<AccountId, Balance>,
    pub consumables_price: Balance,
    /// Items.
    pub weapons: Mapping<AccountId, Balance>,
    pub weapons_price: Balance,
    /// Gun Rights
    pub spells_owned: Mapping<AccountId, bool>,
    /// NFT Skins
    pub nft_skins_owned: Mapping<AccountId, bool>,
    /// Character
    pub character: Mapping<AccountId, Character>,
}

    impl VrmetaxRPG {
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
        self.weapons.insert(caller, &0);
        self.weapons_price = 1;
        self.consumables.insert(caller, &0);
        self.consumables_price = 10;
        self.spells_owned.insert(caller, &false);
        self.nft_skins_owned.insert(caller, &false);
        self.character.insert(caller, &(caller, 1u8, 1u32));
    }

        #[ink(message, payable)]
        pub fn buy_weapons(&mut self, _amount: Balance) {
            let caller: AccountId = self.env().caller();
            let price: Balance = self.weapons_price;

            //assert!(sent_amount >= 1_000000000);

            let amount: Balance = _amount * price;
            assert!(amount >= price, "1 coin or naught");

            let swords: Balance = _amount;

            if self.env().transfer(self.master_address, amount).is_err() {
                panic!(
                    "Funding problem."
                )
            }

            let old_amount = self.weapons.get(&caller);
            if old_amount == None {
                self.weapons.insert(caller, &swords)
            } 
            else {
                self.weapons.insert(caller, &(swords + old_amount.unwrap()));
            }
            
        }

        #[ink(message, payable)]
        pub fn buy_consumables(&mut self, _amount: Balance) {
            let caller: AccountId = self.env().caller();
            let price: Balance = self.consumables_price;

            let amount: Balance = _amount * price;

            assert!(amount >= price, "10 coins or naught");
            let potions: Balance = _amount;

            if self.env().transfer(self.master_address, amount).is_err() {
                panic!(
                    "Funding problem."
                )
            }
            let old_amount = self.consumables.get(&caller);
            if old_amount == None {
                self.consumables.insert(caller, &potions)
            } 
            else {
                self.consumables.insert(caller, &(potions + old_amount.unwrap()));
            }
        }

        #[ink(message, payable)]
        pub fn buy_spell(&mut self) {
            let caller: AccountId = self.env().caller();
            let price: Balance = 50_000_000_000;

            let amount = self.env().transferred_value();
            assert!(amount >= price, "50 coins or naught");

            if self.env().transfer(self.master_address, amount).is_err() {
                panic!(
                    "Funding problem."
                )
            }

            self.spells_owned.insert(caller, &true);
        }

        #[ink(message, payable)]
        pub fn buy_nft_skin(&mut self) {
            let caller: AccountId = self.env().caller();
            let price: Balance = 200_000_000_000;

            let amount = price;
            assert!(amount >= price, "200 coins or naught");

            if self.env().transfer(self.master_address, amount).is_err() {
                panic!(
                    "Funding problem."
                )
            }

            self.nft_skins_owned.insert(caller, &true);
        }


        /// Return ammo
        #[ink(message)]
        pub fn get_weapons(&self) -> Balance {
            let caller: AccountId = self.env().caller();
            let result = self.weapons.get(&caller);
            if result == None {
                return 0
            } 
            else {
                return result.unwrap()
            }
        }

         /// Return ammo
         #[ink(message)]
         pub fn get_consumables(&self) -> Balance {
             let caller: AccountId = self.env().caller();
             let result = self.consumables.get(&caller);
            if result == None {
                return 0
            } 
            else {
                return result.unwrap()
            }
         }

         #[ink(message)]
         pub fn get_spells_owned(&self) -> bool {
             let caller: AccountId = self.env().caller();
             self.spells_owned.get(&caller).unwrap()
         }

         #[ink(message)]
         pub fn get_owns_nft_skin(&self) -> bool {
             let caller: AccountId = self.env().caller();
             self.nft_skins_owned.get(&caller).unwrap()
         }

         #[ink(message)]
         pub fn set_weapons_consumables_prices(&mut self, weapons_price: Balance, consumables_price: Balance) {
            let caller: AccountId = self.env().caller();
            assert!(caller == self.master_address, "Only the Owner can set the Prices");
            self.weapons_price = weapons_price;
            self.consumables_price = consumables_price; 
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

        const DEFAULT_GAS_LIMIT: Balance = 1_000_000;

        fn default_accounts() -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("off-chain environment should have been initialized already")
        }

        fn set_next_caller(caller: AccountId, value: Balance) {
            ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                caller,
                contract_id(),
                DEFAULT_GAS_LIMIT.try_into().unwrap(),
                value,
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])),
            )
        }

        fn get_balance(account_id: AccountId) -> Balance {
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(account_id)
                .expect("Cannot set account balance")
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(account_id, balance)
                .expect("Cannot set account balance");
        }

        fn contract_id() -> AccountId {
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>()
                .expect("Cannot get contract id")
        }

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let vrmetax1 = VrmetaxRPG::new();
            let result = vrmetax1.get_weapons();
            assert_eq!(result, 0);
        }

        #[ink::test]
        fn buy_weapons_works() {
            let mut vrmetax1 = VrmetaxRPG::new();
            let accounts = default_accounts();
            let og_bal = get_balance(vrmetax1.master_address);
            set_next_caller(accounts.alice, 100_000_000_000);
            set_balance(contract_id(), 10_000_000_000);
            vrmetax1.buy_weapons(10);

            let result2 = vrmetax1.get_weapons();
            let result3 = get_balance(vrmetax1.master_address);
            assert_eq!(result2, 10);
            assert_eq!(result3, og_bal + 10_000_000_000);
        }

        #[ink::test]
        fn buy_consumables_works() {
            let mut vrmetax1 = VrmetaxRPG::new();
            let accounts = default_accounts();
            let og_bal = get_balance(vrmetax1.master_address);
            set_next_caller(accounts.alice, 100_000_000_000);
            set_balance(contract_id(), 50_000_000_000);
            vrmetax1.buy_consumables(5);

            let result2 = vrmetax1.get_consumables();
            let result3 = get_balance(vrmetax1.master_address);
            assert_eq!(result2, 5);
            assert_eq!(result3, og_bal + 50_000_000_000);
        }

        #[ink::test]
        fn buy_nft_skin_works() {
            let mut vrmetax1 = VrmetaxRPG::new();
            let accounts = default_accounts();
            let og_bal = get_balance(vrmetax1.master_address);

            let owns1 = vrmetax1.get_owns_nft_skin();
            assert_eq!(owns1, false);

            set_next_caller(accounts.alice, 200_000_000_000);
            set_balance(contract_id(), 200_000_000_000);
            vrmetax1.buy_nft_skin();

            let owns2 = vrmetax1.get_owns_nft_skin();
            let result3 = get_balance(vrmetax1.master_address);

            assert_eq!(owns2, true);
            assert_eq!(result3, og_bal + 200_000_000_000);
        }

        #[ink::test]
        fn set_price_works() {
            let mut vrmetax1 = VrmetaxRPG::new();
            let accounts = default_accounts();
            let og_bal = get_balance(vrmetax1.master_address);
            set_next_caller(accounts.alice, 100_000_000_000);

            set_balance(contract_id(), 10_000_000_000);
            vrmetax1.buy_weapons(10);
            let result2 = vrmetax1.get_weapons();
            let og_bal2 = get_balance(vrmetax1.master_address);
            assert_eq!(result2, 10);
            assert_eq!(og_bal2, og_bal + 10_000_000_000);
            

            vrmetax1.set_weapons_consumables_prices(2_000_000_000, 1_000_000_000);
            set_next_caller(accounts.bob, 100_000_000_000);
            set_balance(contract_id(), 10_000_000_000);
            vrmetax1.buy_weapons(5);
            let result3 = vrmetax1.get_weapons();
            assert_eq!(result3, 5);
            let og_bal3 = get_balance(vrmetax1.master_address);
            assert_eq!(og_bal3, og_bal2 + 10_000_000_000);
         
        }

    }
}


