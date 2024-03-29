//! RPC interface for the `ManualSeal` Engine.

use crate::error::Error;
use futures::{
	channel::{mpsc, oneshot},
	prelude::*,
	stream::StreamExt,
	SinkExt,
};
use jsonrpsee::{
	core::{async_trait, Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
};
use sc_consensus::ImportedAux;
use serde::{Deserialize, Serialize};
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	traits::{Block as BlockT, Header},
	EncodedJustification, SaturatedConversion,
};
use std::sync::Arc;

/// Sender passed to the authorship task to report errors or successes.
pub type Sender<T> = Option<oneshot::Sender<std::result::Result<T, Error>>>;

/// Message sent to the background authorship task, usually by RPC.
pub enum EngineCommand<Hash> {
	/// Tells the engine to propose a new block
	///
	/// if create_empty == true, it will create empty blocks if there are no transactions
	/// in the transaction pool.
	///
	/// if finalize == true, the block will be instantly finalized.
	SealNewBlock {
		/// if true, empty blocks(without extrinsics) will be created.
		/// otherwise, will return Error::EmptyTransactionPool.
		create_empty: bool,
		/// instantly finalize this block?
		finalize: bool,
		/// specify the parent hash of the about-to-created block
		parent_hash: Option<Hash>,
		/// sender to report errors/success to the rpc.
		sender: Sender<CreatedBlock<Hash>>,
	},
	/// Tells the engine to finalize the block with the supplied hash
	FinalizeBlock {
		/// hash of the block
		hash: Hash,
		/// sender to report errors/success to the rpc.
		sender: Sender<()>,
		/// finalization justification
		justification: Option<EncodedJustification>,
	},
}

/// RPC trait that provides methods for interacting with the manual-seal authorship task over rpc.
#[rpc(client, server)]
pub trait ManualSealApi<Block>
where
	Block: BlockT,
{
	/// Instructs the manual-seal authorship task to create a new block
	#[method(name = "engine_createBlock")]
	async fn create_block(
		&self,
		create_empty: bool,
		finalize: bool,
		parent_hash: Option<Block::Hash>,
	) -> RpcResult<CreatedBlock<Block::Hash>>;

	/// Instructs the manual-seal authorship task to finalize a block
	#[method(name = "engine_finalizeBlock")]
	async fn finalize_block(
		&self,
		hash: Block::Hash,
		justification: Option<EncodedJustification>,
	) -> RpcResult<bool>;

	#[method(name = "engine_forwardBlocksTo")]
	async fn forward_blocks_to(
		&self,
		height: <<Block as BlockT>::Header as Header>::Number,
	) -> RpcResult<()>;

	#[method(name = "engine_revertBlocksTo")]
	async fn revert_blocks_to(
		&self,
		height: <<Block as BlockT>::Header as Header>::Number,
	) -> RpcResult<()>;
}

/// A struct that implements the [`ManualSealApiServer`].
pub struct ManualSeal<Block: BlockT, Client, Backend> {
	client: Arc<Client>,
	backend: Arc<Backend>,
	import_block_channel: mpsc::Sender<EngineCommand<Block::Hash>>,
}

/// return type of `engine_createBlock`
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CreatedBlock<Hash> {
	/// hash of the created block.
	pub hash: Hash,
	/// some extra details about the import operation
	pub aux: ImportedAux,
}

impl<Block: BlockT, Client, Backend> ManualSeal<Block, Client, Backend> {
	/// Create new `ManualSeal` with the given reference to the client.
	pub fn new(
		client: Arc<Client>,
		backend: Arc<Backend>,
		import_block_channel: mpsc::Sender<EngineCommand<Block::Hash>>,
	) -> Self {
		Self { client, backend, import_block_channel }
	}
}

#[async_trait]
impl<Block, Client, Backend> ManualSealApiServer<Block> for ManualSeal<Block, Client, Backend>
where
	Block: BlockT,
	Client: sp_api::ProvideRuntimeApi<Block>,
	Client: HeaderBackend<Block>,
	Client: Send + Sync + 'static,
	Backend: sc_client_api::backend::Backend<Block> + Send + Sync + 'static,
{
	async fn create_block(
		&self,
		create_empty: bool,
		finalize: bool,
		parent_hash: Option<Block::Hash>,
	) -> RpcResult<CreatedBlock<Block::Hash>> {
		let mut sink = self.import_block_channel.clone();
		let (sender, receiver) = oneshot::channel();
		// NOTE: this sends a Result over the channel.
		let command = EngineCommand::SealNewBlock {
			create_empty,
			finalize,
			parent_hash,
			sender: Some(sender),
		};

		sink.send(command).await?;

		match receiver.await {
			Ok(Ok(rx)) => Ok(rx),
			Ok(Err(e)) => Err(e.into()),
			Err(e) => Err(JsonRpseeError::to_call_error(e)),
		}
	}

	async fn finalize_block(
		&self,
		hash: Block::Hash,
		justification: Option<EncodedJustification>,
	) -> RpcResult<bool> {
		let mut sink = self.import_block_channel.clone();
		let (sender, receiver) = oneshot::channel();
		let command = EngineCommand::FinalizeBlock { hash, sender: Some(sender), justification };
		sink.send(command).await?;
		receiver.await.map(|_| true).map_err(|e| JsonRpseeError::to_call_error(e))
	}

	async fn forward_blocks_to(
		&self,
		height: <<Block as BlockT>::Header as Header>::Number,
	) -> RpcResult<()> {
		let best_number = self.client.info().best_number;
		if height <= best_number {
			return Err(JsonRpseeError::Custom(
				"Target height is lower than current best height".into(),
			));
		}

		let diff = height - best_number;
		let to_height = (0..diff.saturated_into::<u64>())
			.into_iter()
			.map(|_| EngineCommand::SealNewBlock {
				create_empty: true,
				finalize: false,
				parent_hash: None,
				sender: None,
			})
			.collect::<Vec<EngineCommand<Block::Hash>>>();

		let mut forward_blocks_stream = stream::iter(to_height).map(Ok);

		let mut sink = self.import_block_channel.clone();
		sink.send_all(&mut forward_blocks_stream).await?;

		Ok(())
	}

	async fn revert_blocks_to(
		&self,
		height: <<Block as BlockT>::Header as Header>::Number,
	) -> RpcResult<()> {
		let best_number = self.client.info().best_number;
		if height >= best_number {
			return Err(JsonRpseeError::Custom(
				"Target height is higher than current best height".into(),
			));
		}

		let diff = best_number - height;

		println!("Diff: {:?}", diff);

		let reverted = self
			.backend
			.revert(diff, true)
			.map_err(|e| JsonRpseeError::Custom(format!("Backend Revert Error: {}", e)))?;

		println!("Reverted: {:?}", reverted);

		Ok(())
	}
}

/// report any errors or successes encountered by the authorship task back
/// to the rpc
pub fn send_result<T: std::fmt::Debug>(
	sender: &mut Sender<T>,
	result: std::result::Result<T, crate::Error>,
) {
	if let Some(sender) = sender.take() {
		if let Err(err) = sender.send(result) {
			match err {
				Ok(value) => log::warn!("Server is shutting down: {:?}", value),
				Err(error) => log::warn!("Server is shutting down with error: {}", error),
			}
		}
	} else {
		// No RPC sender sealing/finalization such as instant seal or delayed finalize doesn't
		// report errors over rpc, simply log them.
		match result {
			Ok(r) => log::info!("Consensus with no RPC sender success: {:?}", r),
			Err(e) => log::error!("Consensus with no RPC sender encountered an error: {}", e),
		}
	}
}
