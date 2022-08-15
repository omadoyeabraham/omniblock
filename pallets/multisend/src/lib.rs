#![cfg_attr(not(feature = "std"), no_std)]

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

pub mod weights;
pub use weights::MultiSendWeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::{
		inherent::{Vec},
		sp_runtime::{
			traits::{CheckedSub, Zero},
			DispatchError
		},
		traits::{Currency, ExistenceRequirement, WithdrawReasons},
	};
	use frame_system::pallet_prelude::*;
	use crate::weights::WeightInfo;

	/// The max number of receivers that a user can batch send tokens to at once
	// @TODO Make this configurable from the runtime, but it must not exceed 50
	const MAX_NUMBER_OF_RECEIVERS: u32 = 50;

	type Balance<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Represents a request to transfer tokens to an account
	#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo)]
	// #[scale_info(skip_type_params(T))]
	pub struct TokenTransferRequest<AccountId, Balance> {
		/// The id of the account receiving the tokens
		pub receiver_account: AccountId,

		/// The amount of tokens to be transferred
		pub token_amount: Balance,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The Currency handler for the multisend pallet.
		type Currency: Currency<Self::AccountId>;

		/// Information on runtime weights.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors

	#[pallet::event]
	// #[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TokensSentToMultipleAccountsSuccessfully(),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,

		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

		InsufficientBalance,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(<T as Config>::WeightInfo::send_tokens_to_multiple_receivers())]
		pub fn send_tokens_to_multiple_receivers(
			origin: OriginFor<T>,
			token_transfer_requests: Vec<TokenTransferRequest<T::AccountId, Balance<T>>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			if token_transfer_requests.len() > MAX_NUMBER_OF_RECEIVERS.try_into().unwrap() {
				return Err(DispatchError::Other("Tried to batch send tokens to more than the maximum number of accounts allowed"));
			}

			let total = token_transfer_requests
				.iter()
				.fold(Zero::zero(), |acc: Balance<T>, request| acc + request.token_amount);

			let new_free_balance = T::Currency::free_balance(&sender)
				.checked_sub(&total)
				.ok_or(Error::<T>::InsufficientBalance)?;

			let can_withdraw = T::Currency::ensure_can_withdraw(
				&sender,
				total,
				WithdrawReasons::TRANSFER,
				new_free_balance,
			);

			debug_assert!(can_withdraw.is_ok());

			for transfer_request in token_transfer_requests {
				let _result = T::Currency::transfer(
					&sender,
					&transfer_request.receiver_account,
					transfer_request.token_amount,
					ExistenceRequirement::KeepAlive,
				);
			}

			Ok(())
		}
	}
}
