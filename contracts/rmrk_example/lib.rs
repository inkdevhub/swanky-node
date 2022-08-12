#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::AccountId;
use ink_env::Environment;
use ink_lang as ink;
use ink_prelude::vec::Vec;
use scale::{Decode, Encode};

mod types;
use types::*;

impl From<scale::Error> for RmrkError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
pub enum RmrkError {}

impl ink_env::chain_extension::FromStatusCode for RmrkError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[ink::contract]
mod rmrk {
    use super::*;

    #[ink(storage)]
    pub struct RmrkExample {}

    impl RmrkExample {
        #[ink(constructor)]
        pub fn new() -> Self {
            RmrkExample {}
        }

        // READ functions
        #[ink(message)]
        pub fn next_nft_id(&self, collection_id: CollectionId) -> NftId {
            ::ink_env::chain_extension::ChainExtensionMethod::build(0x00230001)
                .input::<u32>()
                .output::<u32>()
                .ignore_error_code()
                .call(&collection_id)
        }

        #[ink(message)]
        pub fn collection_index(&self) -> CollectionId {
            ::ink_env::chain_extension::ChainExtensionMethod::build(0x00230002)
                .input::<()>()
                .output::<u32>()
                .ignore_error_code()
                .call(&())
        }

        #[ink(message)]
        pub fn next_resource_id(&self, collection_id: CollectionId, nft_id: NftId) -> ResourceId {
            ::ink_env::chain_extension::ChainExtensionMethod::build(0x00230003)
                .input::<(u32, u32)>()
                .output::<u32>()
                .ignore_error_code()
                .call(&(collection_id, nft_id))
        }

        #[ink(message)]
        pub fn collections(&self, collection_id: CollectionId) -> Option<CollectionInfo> {
            ::ink_env::chain_extension::ChainExtensionMethod::build(0x00230004)
                .input::<u32>()
                .output::<u32>()
                .ignore_error_code()
                .call(&collection_id)
        }

        // #[ink(message)]
        // pub fn nfts(&self, collection_id: CollectionId, nft_id: NftId) -> Option<NftInfo> {
            // self.env().extension().nfts(collection_id, nft_id)
        // }

        // #[ink(message)]
        // pub fn priorities(
            // &self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // resource_id: ResourceId,
        // ) -> Option<u32> {
            // self.env()
                // .extension()
                // .priorities(collection_id, nft_id, resource_id)
        // }

        // #[ink(message)]
        // pub fn children(
            // &self,
            // parent: (CollectionId, NftId),
            // child: (CollectionId, NftId),
        // ) -> Option<()> {
            // self.env().extension().children(parent, child)
        // }

        // #[ink(message)]
        // pub fn resources(
            // &self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // resource_id: ResourceId,
        // ) -> Option<ResourceInfo> {
            // self.env()
                // .extension()
                // .resources(collection_id, nft_id, resource_id)
        // }

        // #[ink(message)]
        // pub fn equippable_bases(
            // &self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // base_id: BaseId,
        // ) -> Option<()> {
            // self.env()
                // .extension()
                // .equippable_bases(collection_id, nft_id, base_id)
        // }

        // #[ink(message)]
        // pub fn equippable_slots(
            // &self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // resource_id: ResourceId,
            // base_id: BaseId,
            // slot_id: SlotId,
        // ) -> Option<()> {
            // self.env().extension().equippable_slots(
                // collection_id,
                // nft_id,
                // resource_id,
                // base_id,
                // slot_id,
            // )
        // }

        // #[ink(message)]
        // pub fn properties(
            // &self,
            // collection_id: CollectionId,
            // nft_id: Option<NftId>,
            // key: Vec<u8>,
        // ) -> Option<Vec<u8>> {
            // self.env()
                // .extension()
                // .properties(collection_id, nft_id, key)
        // }

        // #[ink(message)]
        // pub fn lock(&self, collection_id: CollectionId, nft_id: NftId) -> bool {
            // self.env().extension().lock(collection_id, nft_id)
        // }

        // /// write functions
        // #[ink(message)]
        // pub fn mint_ntf(
            // &mut self,
            // owner: AccountId,
            // collection_id: u32,
            // royalty_recipient: Option<AccountId>,
            // royalty: Option<u32>,
            // metadata: Vec<u8>,
            // transferable: bool,
            // resources: Option<Vec<ResourceTypes>>,
        // ) -> Result<(), RmrkError> {
            // self.env().extension().mint_nft(
                // owner,
                // collection_id,
                // royalty_recipient,
                // royalty,
                // metadata,
                // transferable,
                // resources,
            // )
        // }

        // #[ink(message)]
        // pub fn mint_ntf_directly_to_nft(
            // &mut self,
            // owner: (CollectionId, NftId),
            // collection_id: u32,
            // royalty_recipient: Option<AccountId>,
            // royalty: Option<u32>,
            // metadata: Vec<u8>,
            // transferable: bool,
            // resources: Option<Vec<ResourceTypes>>,
        // ) -> Result<(), RmrkError> {
            // self.env().extension().mint_nft_directly_to_nft(
                // owner,
                // collection_id,
                // royalty_recipient,
                // royalty,
                // metadata,
                // transferable,
                // resources,
            // )
        // }

        #[ink(message)]
        pub fn create_collection(
            &mut self,
            metadata: Vec<u8>,
            max: Option<u32>,
            symbol: Vec<u8>,
        ) -> Result<(), RmrkError> {
            ::ink_env::chain_extension::ChainExtensionMethod::build(0x0023000F)
                .input::<(Vec<u8>, Option<u32>, Vec<u8>)>()
                .output::<()>()
                .handle_error_code::<RmrkError>()
                .call(&(metadata, max, symbol))
        }

        // #[ink(message)]
        // pub fn burn_nft(
            // &mut self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // max_burns: u32,
        // ) -> Result<(), RmrkError> {
            // self.env()
                // .extension()
                // .burn_nft(collection_id, nft_id, max_burns)
        // }

        // #[ink(message)]
        // pub fn destroy_collection(&mut self, collection_id: CollectionId) -> Result<(), RmrkError> {
            // self.env().extension().destroy_collection(collection_id)
        // }

        // #[ink(message)]
        // pub fn send(
            // &mut self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // new_owner: AccountIdOrCollectionNftTuple,
        // ) -> Result<(), RmrkError> {
            // self.env()
                // .extension()
                // .send(collection_id, nft_id, new_owner)
        // }

        // #[ink(message)]
        // pub fn accept_nft(
            // &mut self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // new_owner: AccountIdOrCollectionNftTuple,
        // ) -> Result<(), RmrkError> {
            // self.env()
                // .extension()
                // .accept_nft(collection_id, nft_id, new_owner)
        // }

        // #[ink(message)]
        // pub fn reject_nft(
            // &mut self,
            // collection_id: CollectionId,
            // nft_id: NftId,
        // ) -> Result<(), RmrkError> {
            // self.env().extension().reject_nft(collection_id, nft_id)
        // }

        // #[ink(message)]
        // pub fn change_collection_issuer(
            // &mut self,
            // collection_id: CollectionId,
            // new_issuer: AccountId,
        // ) -> Result<(), RmrkError> {
            // self.env()
                // .extension()
                // .change_collection_issuer(collection_id, new_issuer)
        // }

        // #[ink(message)]
        // pub fn set_property(
            // &mut self,
            // collection_id: CollectionId,
            // maybe_nft_id: Option<NftId>,
            // key: Vec<u8>,
            // value: Vec<u8>,
        // ) -> Result<(), RmrkError> {
            // self.env()
                // .extension()
                // .set_property(collection_id, maybe_nft_id, key, value)
        // }

        // #[ink(message)]
        // pub fn lock_collection(&mut self, collection_id: CollectionId) -> Result<(), RmrkError> {
            // self.env().extension().lock_collection(collection_id)
        // }

        // #[ink(message)]
        // pub fn add_basic_resource(
            // &mut self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // resource: BasicResource,
        // ) -> Result<(), RmrkError> {
            // self.env()
                // .extension()
                // .add_basic_resource(collection_id, nft_id, resource)
        // }

        // #[ink(message)]
        // pub fn add_composable_resource(
            // &mut self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // resource: ComposableResource,
        // ) -> Result<(), RmrkError> {
            // self.env()
                // .extension()
                // .add_composable_resource(collection_id, nft_id, resource)
        // }

        // #[ink(message)]
        // pub fn add_slot_resource(
            // &mut self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // resource: SlotResource,
        // ) -> Result<(), RmrkError> {
            // self.env()
                // .extension()
                // .add_slot_resource(collection_id, nft_id, resource)
        // }

        // #[ink(message)]
        // pub fn accept_resource(
            // &mut self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // resource_id: ResourceId,
        // ) -> Result<(), RmrkError> {
            // self.env()
                // .extension()
                // .accept_resource(collection_id, nft_id, resource_id)
        // }

        // #[ink(message)]
        // pub fn remove_resource(
            // &mut self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // resource_id: ResourceId,
        // ) -> Result<(), RmrkError> {
            // self.env()
                // .extension()
                // .remove_resource(collection_id, nft_id, resource_id)
        // }

        // #[ink(message)]
        // pub fn accept_resource_removal(
            // &mut self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // resource_id: ResourceId,
        // ) -> Result<(), RmrkError> {
            // self.env()
                // .extension()
                // .accept_resource_removal(collection_id, nft_id, resource_id)
        // }

        // #[ink(message)]
        // pub fn set_priority(
            // &mut self,
            // collection_id: CollectionId,
            // nft_id: NftId,
            // priorities: Vec<ResourceId>,
        // ) -> Result<(), RmrkError> {
            // self.env()
                // .extension()
                // .set_priority(collection_id, nft_id, priorities)
        // }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
        }
    }
}
