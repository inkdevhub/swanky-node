// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! RPC interface for the transaction payment pallet.

use std::{convert::TryInto, sync::Arc};

use codec::Codec;
use jsonrpsee::{
	core::{RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use pallet_balances_rpc_runtime_api::AccountData;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_rpc::number::NumberOrHex;
use sp_runtime::traits::{Block as BlockT, MaybeDisplay};
use std::marker::{PhantomData, Send, Sync};

pub use pallet_balances_rpc_runtime_api::BalancesApi as BalancesRuntimeApi;

/// RPC trait that provides methods for interacting with the dev balances functionalities.
#[rpc(client, server)]
pub trait BalancesApi<BlockHash, AccountId, Balance> {
	#[method(name = "balance_getAccount")]
	fn get_account(
		&self,
		account_id: AccountId,
		at: Option<BlockHash>,
	) -> RpcResult<AccountData<Balance>>;
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
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> Balances<C, P> {
	/// Creates a new instance of the TransactionPayment Rpc helper.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: PhantomData::default() }
	}
}

impl<Client, Block, AccountId, Balance>
	BalancesApiServer<<Block as BlockT>::Hash, AccountId, Balance> for Balances<Client, Block>
where
	Block: BlockT,
	Client: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	Client::Api: BalancesRuntimeApi<Block, AccountId, Balance>,
	AccountId: Clone + MaybeDisplay + Codec + Send + 'static,
	Balance: Codec + MaybeDisplay + Copy + TryInto<NumberOrHex> + Send + Sync + 'static,
{
	fn get_account(
		&self,
		account_id: AccountId,
		at: Option<Block::Hash>,
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
}
