//! Extensions for manual seal to produce blocks valid for any runtime.
use super::Error;

use sc_consensus::BlockImportParams;
use sp_inherents::InherentData;
use sp_runtime::{traits::Block as BlockT, Digest};

pub mod aura;
pub mod babe;
pub mod timestamp;

/// Consensus data provider, manual seal uses this trait object for authoring blocks valid
/// for any runtime.
pub trait ConsensusDataProvider<B: BlockT>: Send + Sync {
	/// Block import transaction type
	type Transaction;

	/// The proof type.
	type Proof;

	/// Attempt to create a consensus digest.
	fn create_digest(&self, parent: &B::Header, inherents: &InherentData) -> Result<Digest, Error>;

	/// Set up the necessary import params.
	fn append_block_import(
		&self,
		parent: &B::Header,
		params: &mut BlockImportParams<B, Self::Transaction>,
		inherents: &InherentData,
		proof: Self::Proof,
	) -> Result<(), Error>;
}
