//!
use super::Runtime;
pub use pallet_chain_extension_assets::AssetsExtension;
/// Registered WASM contracts chain extensions.
use pallet_contracts::chain_extension::RegisteredChainExtension;

// Following impls defines chain extension IDs.

impl RegisteredChainExtension<Runtime> for AssetsExtension<Runtime> {
	const ID: u16 = 2;
}
