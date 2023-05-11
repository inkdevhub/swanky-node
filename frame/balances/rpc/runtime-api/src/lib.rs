#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
pub use pallet_balances::AccountData;

sp_api::decl_runtime_apis! {
	pub trait BalancesApi<AccountId, Balance>
	where
		AccountId: Codec,
		Balance: Codec,
	{
		fn account(account_id: AccountId) -> AccountData<Balance>;
	}
}
