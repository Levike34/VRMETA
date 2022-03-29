#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;



#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*};
	use frame_system::pallet_prelude::*;
    use frame_support::traits::{UnixTime, Currency};

  ///  use sp_arithmetic::traits::SaturatedConversion;

   pub type NegativeImbalanceOf<T> = <<T as Config>::Vrmeta as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;
   pub type BalanceOf<T> = <<T as Config>::Vrmeta as Currency<<T as frame_system::Config>::AccountId>>::Balance;

   impl<T: Config> Pallet<T> {
    pub fn switch(i: u32) -> BalanceOf<T> {
        i.into()
    }

}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        // The type used to store balances.
        type TimeProvider: UnixTime;
        // Balances
        type Vrmeta: Currency<Self::AccountId>;    
         
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when connected. [who, timestamp]
        Connected(T::AccountId, u64),
        /// Event emitted when disconnected. [who, mined]
        Disconnected(T::AccountId, u64, BalanceOf<T>),
    }

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		  /// Self.
		  AlreadyConnected,
		  /// self
		  NotConnected,
	
	}

	#[pallet::storage]
    /// Maps each proof to its owner and block number when the proof was made
    pub type Players<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        u64,
        OptionQuery,
    >;

  
	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {


        #[pallet::weight(1_000)]
        pub fn connect(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has not already been claimed.
            //ensure!(Players::<T>::get(&sender), Error::<T>::AlreadyConnected);

            // Get the block number from the FRAME System pallet.
            let current_time: u64 = T::TimeProvider::now().as_secs();
          

            // Store the proof with the sender and block number.
            Players::<T>::insert(&sender, current_time);

            // Emit an event that the claim was created.
            Self::deposit_event(Event::Connected(sender, current_time));

            Ok(())
        }

        #[pallet::weight(1_000)]
        pub fn disconnect(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/v3/runtime/origins
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has been claimed.
            //ensure!(!Players::<T>::get(&sender), Error::<T>::NotConnected);
        
            let reward = Players::<T>::get(&sender).unwrap();
            let current_time: u64 = T::TimeProvider::now().as_secs();
            let time_played = current_time - reward;
            let fix_time: u32 = time_played as u32;
            let multiplier: BalanceOf<T> = Self::switch(fix_time);
            let coin_per_hour: BalanceOf<T> = Self::switch(1_000_000_000);
            let divisor: BalanceOf<T> = Self::switch(3_600_000);

            //let issue: T::Vrmeta<Self:AccountId> = 1_000_000_000;
            let amount_to_give: BalanceOf<T> = (multiplier / divisor) * coin_per_hour;       
            let tx = T::Vrmeta::deposit_into_existing(&sender, amount_to_give);

            //let reward_to_pay = (time_played / 3_600_000u64) * 1_000_000_000u64;

            Players::<T>::remove(&sender);
            Self::deposit_event(Event::Disconnected(sender, time_played, amount_to_give));
            Ok(())
        }
       
        
        }

        
   
}
