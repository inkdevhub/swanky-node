#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
pub use pallet_balances::AccountData;
use sp_runtime::traits::Block as BlockT;

sp_api::decl_runtime_apis! {
	pub trait BalancesApi<AccountId, Balance>
	where
		AccountId: Codec,
		Balance: Codec,
	{
		fn account(account_id: AccountId) -> AccountData<Balance>;

		fn get_set_free_balance_extrinsic(account_id: AccountId, free_balance: Balance) -> <Block as BlockT>::Extrinsic;
	}
}
