#![cfg_attr(not(feature = "std"), no_std)]
use sp_runtime::{DispatchError, Permill};

use chain_extension_traits::ChainExtensionExec;

use codec::Encode;
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use pallet_contracts::chain_extension::{Environment, Ext, InitState, SysConfig, UncheckedFrom};
use pallet_rmrk_core::BoundedResourceTypeOf;
use rmrk_traits::{
	primitives::{CollectionId, NftId, ResourceId},
	BasicResource,
};
use sp_std::{marker::PhantomData, vec::Vec};
use rmrk_chain_extension_types::RmrkFunc;


pub struct RmrkExtension<R>(PhantomData<R>);

impl<
		T: pallet_rmrk_core::Config
			+ pallet_uniques::Config<CollectionId = CollectionId, ItemId = NftId>,
	> ChainExtensionExec<T> for RmrkExtension<T>
{
	fn execute_func<E>(func_id: u32, env: Environment<E, InitState>) -> Result<(), DispatchError>
	where
		E: Ext<T = T>,
		<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{
		let func_id = RmrkFunc::try_from(func_id)?;

		match func_id {
			RmrkFunc::NextNftId => {
				let mut env = env.buf_in_buf_out();
				let collection_id: u32 = env.read_as()?;

				let nft_id = pallet_rmrk_core::Pallet::<T>::next_nft_id(collection_id);
				let nft_id_encoded = nft_id.encode();

				env.write(&nft_id_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write next_nft_id")
				})?;
			},

			RmrkFunc::CollectionIndex => {
				let mut env = env.buf_in_buf_out();
				let index = pallet_rmrk_core::Pallet::<T>::collection_index();
				let index_encoded = index.encode();

				env.write(&index_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write collection_index")
				})?;
			},

			RmrkFunc::NextResourceId => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id): (T::CollectionId, T::ItemId) = env.read_as()?;

				let resource_id =
					pallet_rmrk_core::Pallet::<T>::next_resource_id(collection_id, nft_id);
				let resource_id_encoded = resource_id.encode();

				env.write(&resource_id_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write next_resource_id")
				})?;
			},

			RmrkFunc::Collections => {
				let mut env = env.buf_in_buf_out();
				let collection_id: T::CollectionId = env.read_as()?;

				let collections = pallet_rmrk_core::Pallet::<T>::collections(collection_id);
				let collections_encoded = collections.encode();

				env.write(&collections_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write collections_encoded")
				})?;
			},

			RmrkFunc::Nfts => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id): (T::CollectionId, T::ItemId) = env.read_as()?;

				let nfts = pallet_rmrk_core::Pallet::<T>::nfts(collection_id, nft_id);
				let nfts_encoded = nfts.encode();

				env.write(&nfts_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write nfts")
				})?;
			},

			RmrkFunc::Resources => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, resource_id): (T::CollectionId, T::ItemId, ResourceId) =
					env.read_as()?;

				let resources =
					pallet_rmrk_core::Pallet::<T>::resources((collection_id, nft_id, resource_id));
				let resources_encoded = resources.encode();

				env.write(&resources_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write resources_encoded")
				})?;
			},

			RmrkFunc::MintNft => {
				let mut env = env.buf_in_buf_out();
				let (
					beneficiary,
					collection_id,
					royalty_recipient,
					royalty,
					metadata,
					transferable,
					resources,
				): (
					T::AccountId,
					T::CollectionId,
					Option<T::AccountId>,
					Option<Permill>,
					Vec<u8>,
					bool,
					Option<BoundedResourceTypeOf<T>>,
				) = env.read_as_unbounded(env.in_len())?;

				let caller = env.ext().caller().clone();
				pallet_rmrk_core::Pallet::<T>::mint_nft(
					RawOrigin::Signed(caller).into(),
					Some(beneficiary.clone()),
					collection_id,
					royalty_recipient,
					royalty,
					metadata.try_into().unwrap(),
					transferable,
					resources,
				)?;
			},

			RmrkFunc::CreateCollection => {
				let mut env = env.buf_in_buf_out();
				let (metadata, max, symbol): (
					Vec<u8>,
					Option<u32>,
					BoundedVec<u8, T::CollectionSymbolLimit>,
				) = env.read_as_unbounded(env.in_len())?;

				let caller = env.ext().caller().clone();
				pallet_rmrk_core::Pallet::<T>::create_collection(
					RawOrigin::Signed(caller).into(),
					metadata.try_into().unwrap(),
					max,
					symbol.try_into().unwrap(),
				)?;
			},

			RmrkFunc::AddBasicResource => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, resource): (
					T::CollectionId,
					T::ItemId,
					BasicResource<BoundedVec<u8, T::StringLimit>>,
				) = env.read_as_unbounded(env.in_len())?;

				let caller = env.ext().caller().clone();
				pallet_rmrk_core::Pallet::<T>::add_basic_resource(
					RawOrigin::Signed(caller).into(),
					collection_id,
					nft_id,
					resource,
				)?;
			},

			RmrkFunc::RemoveResource => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, resource_id): (T::CollectionId, T::ItemId, ResourceId) =
					env.read_as()?;

				let caller = env.ext().caller().clone();
				pallet_rmrk_core::Pallet::<T>::remove_resource(
					RawOrigin::Signed(caller).into(),
					collection_id,
					nft_id,
					resource_id,
				)?;
			},
		}

		Ok(())
	}
}
