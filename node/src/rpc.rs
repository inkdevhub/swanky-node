//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use futures::channel::mpsc::Sender;
use jsonrpsee::RpcModule;
use sp_runtime::traits::Block as BlockT;
use swanky_runtime::{opaque::Block, AccountId, Balance, Hash, Index};

use sc_consensus_manual_seal::{
	rpc::{ManualSeal, ManualSealApiServer},
	EngineCommand,
};
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// Full client dependencies.
pub struct FullDeps<C, B, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// The backend instance to use.
	pub backend: Arc<B>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// A command stream to send authoring commands to manual seal consensus engine
	pub command_sink: Sender<EngineCommand<Hash>>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, B, P>(
	deps: FullDeps<C, B, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: pallet_balances_rpc::BalancesRuntimeApi<Block, AccountId, Balance>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool<Block = Block, Hash = <Block as BlockT>::Hash> + 'static,
	B: sc_client_api::backend::Backend<Block> + Send + Sync + 'static,
	P: TransactionPool + 'static,
{
	use pallet_balances_rpc::{Balances, BalancesApiServer};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut io = RpcModule::new(());
	let FullDeps { client, backend, pool, deny_unsafe, command_sink } = deps;

	io.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	io.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	io.merge(Balances::new(client.clone(), pool.clone()).into_rpc())?;

	// The final RPC extension receives commands for the manual seal consensus engine.
	io.merge(ManualSeal::new(client, backend, command_sink).into_rpc())?;

	Ok(io)
}
