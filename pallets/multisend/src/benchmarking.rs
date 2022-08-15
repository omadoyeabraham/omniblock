//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use frame_system::RawOrigin;
use crate::Pallet as MultiSend;
use frame_benchmarking::{account, benchmarks};
use frame_support::{inherent::Vec, traits::Currency};

const SEED: u32 = 0;
const NUMBER_OF_RECEIVING_ACCOUNTS: u32 = 50;

/// Grab a funded user.
fn create_funded_user<T: Config>(
	string: &'static str,
	n: u32,
	balance_factor: u32,
) -> T::AccountId {
	let user = account(string, n, SEED);
	let balance = T::Currency::minimum_balance() * balance_factor.into();
	let _ = T::Currency::make_free_balance_be(&user, balance);
	user
}

// Loop and do the following an arbitrary number of times (e.g n = 100), each time measuring the execution time
// Setup
// 		-- Create and fund the sending account with (n+1) * 10 tokens, this allows it to send 10 tokens to n accounts and still have some tokens leftover
//		-- Create n number of receiving accounts with an existensial deposit

// Action
// 		-- Call the send_tokens_to_multiple_receivers extrinsic from the pallet

// Verify
//		-- Ascertain that the sending account only has 10 tokens left
//		-- Ascertain that the receiving accounts all received their 10 tokens

benchmarks! {
	send_tokens_to_multiple_receivers {
		let sender = create_funded_user::<T>("user", SEED, (NUMBER_OF_RECEIVING_ACCOUNTS + 1) * 100);
		let sender_initial_balance =  T::Currency::free_balance(&sender);
		let mut token_transfer_requests = Vec::new();

		for i in 1 .. NUMBER_OF_RECEIVING_ACCOUNTS {
			token_transfer_requests.push(
				TokenTransferRequest {
					receiver_account: create_funded_user::<T>("receiver", i, 1),
					token_amount: T::Currency::minimum_balance() + 10u32.into() } // Send 10 tokens
			);
		}

	}: _(RawOrigin::Signed(sender.clone()), token_transfer_requests.clone())
	verify {
		assert_eq!(true, T::Currency::free_balance(&sender) < sender_initial_balance);
		for request in token_transfer_requests {
			assert_eq!(T::Currency::minimum_balance() + T::Currency::minimum_balance() + 10u32.into(), T::Currency::free_balance(&request.receiver_account)) // expect 11 tokens = 1 minimum_balance + 10 sent
		}
	}

	impl_benchmark_test_suite!(MultiSend, crate::mock::new_test_ext(), crate::mock::Test);
}
