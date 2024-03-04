use std::{convert::TryInto, sync::Arc};

use codec::Codec;
use futures::future::TryFutureExt;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use pallet_balances_rpc_runtime_api::AccountData;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_rpc::number::NumberOrHex;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, MaybeDisplay},
};
use std::marker::{Send, Sync};

pub use pallet_balances_rpc_runtime_api::BalancesApi as BalancesRuntimeApi;

/// RPC trait that provides methods for interacting with the dev balances functionalities.
#[rpc(server)]
#[async_trait]
pub trait BalancesApi<BlockHash, AccountId, Balance> {
	#[method(name = "balance_getAccount")]
	fn get_account(
		&self,
		account_id: AccountId,
		at: Option<BlockHash>,
	) -> RpcResult<AccountData<Balance>>;

	#[method(name = "balance_setFreeBalance")]
	async fn set_free_balance(&self, account_id: AccountId, free_balance: Balance)
		-> RpcResult<()>;
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

/// Provides RPC methods to query a dispatchable's class, weight and fee.
pub struct Balances<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	/// Shared reference to the transaction pool.
	pool: Arc<P>,
}

impl<C, P> Balances<C, P> {
	/// Creates a new instance of the TransactionPayment Rpc helper.
	pub fn new(client: Arc<C>, pool: Arc<P>) -> Self {
		Self { client, pool }
	}
}

#[async_trait]
impl<Client, Pool, AccountId, Balance>
	BalancesApiServer<<Pool::Block as BlockT>::Hash, AccountId, Balance> for Balances<Client, Pool>
where
	Client: Send + Sync + 'static + ProvideRuntimeApi<Pool::Block> + HeaderBackend<Pool::Block>,
	Client::Api: BalancesRuntimeApi<Pool::Block, AccountId, Balance>,
	Pool: TransactionPool + 'static,
	AccountId: Clone + MaybeDisplay + Codec + Send + 'static,
	Balance: Codec + MaybeDisplay + Copy + TryInto<NumberOrHex> + Send + Sync + 'static,
{
	fn get_account(
		&self,
		account_id: AccountId,
		at: Option<<Pool::Block as BlockT>::Hash>,
	) -> RpcResult<AccountData<Balance>> {
		let runtime_api = self.client.runtime_api();
		let at_hash = at.unwrap_or_else(|| self.client.info().best_hash);

		let account_data = runtime_api.account(at_hash, account_id).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				Error::DecodeError.into(),
				"Unable to get account data.",
				Some(e.to_string()),
			))
		})?;

		Ok(account_data)
	}

	async fn set_free_balance(
		&self,
		account_id: AccountId,
		free_balance: Balance,
	) -> RpcResult<()> {
		let best_block_hash = self.client.info().best_hash;

		// TODO: Find a way to construct Balances Call which can casted to `<<Pool as
		// TransactionPool>::Block as BlockT>::Extrinsic` without using runtime_api. Is that
		// possible?
		let extrinsic: <<Pool as TransactionPool>::Block as BlockT>::Extrinsic = match self
			.client
			.runtime_api()
			.get_set_free_balance_extrinsic(best_block_hash, account_id, free_balance)
		{
			Ok(extrinsic) => extrinsic,
			Err(_) => return RpcResult::Err(internal_err("cannot access runtime api")),
		};

		self.pool
			.submit_one(
				&BlockId::Hash(best_block_hash),
				sc_transaction_pool_api::TransactionSource::Local,
				extrinsic,
			)
			.map_ok(move |_| ())
			.map_err(|err| internal_err(err))
			.await
	}
}

pub fn err<T: ToString>(code: i32, message: T, data: Option<&[u8]>) -> jsonrpsee::core::Error {
	jsonrpsee::core::Error::Call(jsonrpsee::types::error::CallError::Custom(
		jsonrpsee::types::error::ErrorObject::owned(
			code,
			message.to_string(),
			data.map(|bytes| {
				jsonrpsee::core::to_json_raw_value(&format!("0x{}", hex::encode(bytes)))
					.expect("fail to serialize data")
			}),
		),
	))
}

pub fn internal_err<T: ToString>(message: T) -> jsonrpsee::core::Error {
	err(jsonrpsee::types::error::INTERNAL_ERROR_CODE, message, None)
}

pub fn internal_err_with_data<T: ToString>(message: T, data: &[u8]) -> jsonrpsee::core::Error {
	err(jsonrpsee::types::error::INTERNAL_ERROR_CODE, message, Some(data))
}
