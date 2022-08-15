use super::*;
use crate::mock::*;

#[test]
fn params_should_work() {
	new_test_ext().execute_with(|| {
		assert_eq!(Balances::free_balance(42), 0);
		assert_eq!(Balances::free_balance(1), 1000);
		assert_eq!(Balances::free_balance(6), 10);
		assert_eq!(Balances::total_issuance(), 1050);
	});
}

#[test]
fn it_sends_tokens_to_multiple_users_simultaneously() {
	new_test_ext().execute_with(|| {
		let transfers = vec![
			TokenTransferRequest { receiver_account: 2, token_amount: 100 },
			TokenTransferRequest { receiver_account: 6, token_amount: 100 },
		];

		MultiSend::send_tokens_to_multiple_receivers(Origin::signed(1), transfers).unwrap();

		assert_eq!(Balances::free_balance(1), 800);
		assert_eq!(Balances::free_balance(2), 110);
		assert_eq!(Balances::free_balance(6), 110);
	});
}

#[test]
fn it_gives_correct_error_if_sender_balance_is_insufficient() {
	new_test_ext().execute_with(|| {
		let transfers = vec![
			TokenTransferRequest { receiver_account: 2, token_amount: 1000 },
			TokenTransferRequest { receiver_account: 6, token_amount: 1000 },
		];

		assert!(MultiSend::send_tokens_to_multiple_receivers(Origin::signed(1), transfers).is_err());
	});
}
//
// fn it_gives_correct_error_if_invalid__receiver_account_is_provided() {
// 	todo!()
// }
