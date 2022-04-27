#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod pixellandmap {

use ink_storage::traits::SpreadAllocate;
use ink_storage::Mapping;

pub type Coords = [u32; 2];
pub type Grid = [Coords; 2];

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    /// Returned if caller is not the owner.
    NonOwner,
    /// Out of Bounds
    OutOfBounds,
    /// Already Owned
    AlreadyOwned,
    /// Not for sale
    NotForSale
}

pub type Result<T> = core::result::Result<T, Error>;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
#[ink(storage)]
#[derive(SpreadAllocate)]
   pub struct Pixellandmap {
    /// Plots owned.
    pub plot: Mapping<AccountId, Grid>,
    /// Price per Plot
    pub plot_price: Balance,
    /// Master wallet who receives funds from game.
    pub owner: AccountId,
    /// Mapsize
    pub map_size: Grid,
    /// Ownership Tracker
    pub is_owned: Mapping<Coords, bool>,
    pub is_owner: Mapping<Coords, AccountId>,
}

    impl Pixellandmap {
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
        self.plot.insert(caller, &[[0,0], [10,10]]);
        self.plot_price = 100_000_000_000;
        self.map_size = [[0, 0], [100, 100]];

        let mut x2: u32 = 0;
        let mut y2: u32 = 0;
        while y2 <= 10 {
            while x2 <= 10 {
                self.is_owned.insert([x2, y2], &true);
                self.is_owner.insert([x2, y2], &caller);
                x2 += 1;
            }
            y2 += 1;
        }

        
    }

    #[ink(message, payable)]
    pub fn buy_new_plot(&mut self, coords: Grid) -> Result<()>  {
        let caller: AccountId = self.env().caller();
        let result = self.check_if_owned(coords);
        if result == true {
            return Err(Error::AlreadyOwned)
        } else {
            let mut x = coords[0][0];
            let mut y = coords[0][1];
            let x2 = coords[1][0];
            let y2 = coords[1][1];
            while y <= y2 {
                while x <= x2 {
                    self.is_owned.insert([x, y], &true);
                    self.is_owner.insert([x, y], &caller);
                    x += 1;
                }
                y += 1;
            }
         
        }
        if self.env().transfer(self.owner, self.plot_price).is_err() {
            panic!(
                "Funding problem."
            )
        }
        Ok(())
    }

        #[ink(message)]
        pub fn get_plot(&mut self, who: AccountId) -> Grid {
            let caller: AccountId = self.env().caller();
            self.plot.get(&caller).unwrap()
        }

        #[ink(message)]
        pub fn get_map_size(&mut self) -> Grid {
            self.map_size
        }

        #[ink(message)]
        pub fn check_if_owned(&mut self, coords: Grid) -> bool {
            let mut x = coords[0][0];
            let mut y = coords[0][1];
            let x2 = coords[1][0];
            let y2 = coords[1][1];
            while y <= y2 {
                while x <= x2 {
                    
                    let option = self.is_owned.get(&[x, y]);
                    if option == Some(true) {
                        return true
                    } 
                    else {
                        x += 1; 
                    }
                }
                y += 1;
            }
            return false
        }

        #[ink(message)]
        pub fn get_owner(&mut self, coords: Coords) -> AccountId {
            let result = self.is_owner.get(coords).unwrap();
            result.into()
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
            let pixellandmap = Pixellandmap::new();
            let result = pixellandmap.map_size;
            assert_eq!(result, [[0,0], [100,100]]);
        }

        #[ink::test]
        fn ownership_works() {
            let mut pixellandmap = Pixellandmap::new();
            let result = pixellandmap.check_if_owned([[0,0], [10, 2]]);
            assert_eq!(result, true);
            let result2 = pixellandmap.check_if_owned([[20,0], [50, 1]]);
            assert_eq!(result2, false);
        }

        #[ink::test]
        fn get_owner_works() {
            let mut pixellandmap = Pixellandmap::new();
            let result = pixellandmap.get_plot(pixellandmap.owner);
            let result2 = pixellandmap.get_owner(result[0]);
            assert_eq!(result2, pixellandmap.owner);
           
        }

        #[ink::test]
        fn buy_works() {
            let mut pixellandmap = Pixellandmap::new();
            
            let og_bal = get_balance(pixellandmap.owner);
            let accounts = default_accounts();
            set_next_caller(accounts.alice, 100_000_000_000);
            set_balance(contract_id(), 100_000_000_000);
            let result = pixellandmap.check_if_owned([[11, 11], [22, 22]]);
            assert_eq!(result, false);

            let tx = pixellandmap.buy_new_plot([[11,11], [22, 22]]);
            let result2 = pixellandmap.check_if_owned([[11, 11], [22, 22]]);
            let owned = pixellandmap.get_owner([11, 11]);
            assert_eq!(result2, true);
            assert_eq!(owned, accounts.alice);
            let result3 = get_balance(pixellandmap.owner);
            assert_eq!(result3, og_bal + 100_000_000_000);
           
        }


    }
}


