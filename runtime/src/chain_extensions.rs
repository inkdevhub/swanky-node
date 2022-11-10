//!
use super::Runtime;
/// Registered WASM contracts chain extensions.
use pallet_contracts::chain_extension::RegisteredChainExtension;

pub use pallet_chain_extension_dapps_staking::DappsStakingExtension;
pub use pallet_chain_extension_rmrk::RmrkExtension;

// Following impls defines chain extension IDs.

impl RegisteredChainExtension<Runtime> for DappsStakingExtension<Runtime> {
	const ID: u16 = 0x0000;
}

impl RegisteredChainExtension<Runtime> for RmrkExtension<Runtime> {
	const ID: u16 = 0x0001;
}
