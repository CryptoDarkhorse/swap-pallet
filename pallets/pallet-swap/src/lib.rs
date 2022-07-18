#![cfg_attr(not(feature = "std"), no_std)]

pub use codec::{Decode, Encode};
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(Clone, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Swap<AccountId, TokenId> {
    // The token being swapped.
    token_id: TokenId,
    // The "swap token" id.
    swap_token: TokenId,
    // This swap account.
    account: AccountId,
}

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_erc20::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://docs.substrate.io/v3/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn something)]
    // Learn more about declaring storage items:
    // https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
    pub type Something<T> = StorageValue<_, u32>;

    // #[pallet::storage]
    // #[pallet::getter(fn something)]
    // pub type Something<T> = StorageValue<_, u32>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, T::AccountId),

        // parameters: [token_id, amount, price, owner]
        SellOrderCreated(u128, u128, u128, T::AccountId),
        // parameters: [order_id, status]
        SellOrderCancelled(u128, bool),
        // parameters: [order_id, amount of token sold, amount paid, seller, buyer]
        BuyOrderFilled(u128, u128, u128, T::AccountId, T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,

        /// Deadline hit.
        Deadline,
        /// Zero tokens supplied.
        ZeroTokens,
        /// Zero reserve supplied.
        ZeroAmount,
        /// No Swap exists at this Id.
        NoSwapExists,
        /// A Swap already exists for a particular TokenId.
        SwapAlreadyExists,
        /// Requested zero liquidity.
        RequestedZeroLiquidity,
        /// Would add too many tokens to liquidity.
        TooManyTokens,
        /// Not enough liquidity created.
        TooLowLiquidity,
        /// No currency is being swapped.
        NoCurrencySwapped,
        /// No tokens are being swapped.
        NoTokensSwapped,
        /// Trying to burn zero shares.
        BurnZeroShares,
        /// No liquidity in the swap.
        NoLiquidity,
        /// Not enough currency will be returned.
        NotEnoughCurrency,
        /// Not enough tokens will be returned.
        NotEnoughTokens,
        /// Swap would cost too much in currency.
        TooExpensiveCurrency,
        /// Swap would cost too much in tokens.
        TooExpensiveTokens,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Create a sell order
        #[pallet::weight(10_000)]
        pub fn create_sell_order(
            origin: OriginFor<T>,
            token_id: u128,
            volume: u128,
            price: u128,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // TODO: implement

            Self::deposit_event(Event::SellOrderCreated(token_id, volume, price, who));
            Ok(())
        }

        // Cancel
        #[pallet::weight(10_000)]
        pub fn cancel_sell_order(origin: OriginFor<T>, order_id: u128) -> DispatchResult {
            let _owner = ensure_signed(origin)?;

            // TODO: implement

            Self::deposit_event(Event::SellOrderCancelled(order_id, true));
            Ok(())
        }

        // Buy
        #[pallet::weight(10_000)]
        pub fn buy_order(
            origin: OriginFor<T>,
            _project_id: u128,
            _bundle_id: u128,
            volume: u128,
        ) -> DispatchResult {
            let buyer = ensure_signed(origin)?;

            // TODO: implement

            let price: u128 = 0;
            let order_id: u128 = 0;
            let seller: T::AccountId = buyer.clone();

            Self::deposit_event(Event::BuyOrderFilled(
                order_id, volume, price, seller, buyer,
            ));

            Ok(())
        }
    }
}
