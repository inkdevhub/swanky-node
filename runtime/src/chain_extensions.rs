//!
use super::Runtime;
pub use pallet_chain_extension_assets::weights::WeightInfo;
/// Registered WASM contracts chain extensions.
use pallet_contracts::chain_extension::RegisteredChainExtension;
pub use pallet_chain_extension_assets::AssetsExtension;
pub use pallet_chain_extension_dapps_staking::DappsStakingExtension;

// Following impls defines chain extension IDs.

impl RegisteredChainExtension<Runtime> for DappsStakingExtension<Runtime> {
	const ID: u16 = 0x0000;
}


/// Based on chain-extension registry https://github.com/paritytech/chainextension-registry
impl<W: WeightInfo> RegisteredChainExtension<Runtime> for AssetsExtension<Runtime, W> {
	const ID: u16 = 18678;
}