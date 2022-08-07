#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::AccountId;
use ink_env::Environment;
use ink_lang as ink;
use ink_prelude::vec::Vec;
use scale::{Decode, Encode};

mod types;
use types::*;

#[derive(Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RmrkErrorCode {
    Failed,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RmrkError {
    ErrorCode(RmrkErrorCode),
}

impl From<RmrkErrorCode> for RmrkError {
    fn from(error_code: RmrkErrorCode) -> Self {
        Self::ErrorCode(error_code)
    }
}

impl From<scale::Error> for RmrkError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

impl ink_env::chain_extension::FromStatusCode for RmrkErrorCode {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::Failed),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = RmrkExt;
}

#[ink::chain_extension]
pub trait RmrkExt {
    type ErrorCode = RmrkErrorCode;

    // READ functions
    #[ink(extension = 3501, returns_result = false, handle_status = false)]
    fn next_nft_id(collection_id: CollectionId) -> NftId;

    #[ink(extension = 3502, returns_result = false, handle_status = false)]
    fn collection_index() -> CollectionId;

    #[ink(extension = 3503, returns_result = false, handle_status = false)]
    fn next_resource_id(collection_id: CollectionId, nft_id: NftId) -> ResourceId;

    #[ink(extension = 3504, returns_result = false, handle_status = false)]
    fn collections(collection_id: CollectionId) -> Option<CollectionInfo>;

    #[ink(extension = 3505, returns_result = false, handle_status = false)]
    fn nfts(collection_id: CollectionId, nft_id: NftId) -> Option<NftInfo>;

    #[ink(extension = 3506, returns_result = false, handle_status = false)]
    fn priorities(
        collection_id: CollectionId,
        nft_id: NftId,
        resource_id: ResourceId,
    ) -> Option<u32>;

    #[ink(extension = 3507, returns_result = false, handle_status = false)]
    fn children(parent: (CollectionId, NftId), child: (CollectionId, NftId)) -> Option<()>;

    #[ink(extension = 3508, returns_result = false, handle_status = false)]
    fn resources(
        collection_id: u32,
        nft_id: NftId,
        resource_id: ResourceId,
    ) -> Option<ResourceInfo>;

    #[ink(extension = 3509, returns_result = false, handle_status = false)]
    fn equippable_bases(collection_id: CollectionId, nft_id: NftId, base_id: BaseId) -> Option<()>;

    #[ink(extension = 3510, returns_result = false, handle_status = false)]
    fn equippable_slots(
        collection_id: CollectionId,
        nft_id: NftId,
        resource_id: ResourceId,
        base_id: BaseId,
        slot_id: SlotId,
    ) -> Option<()>;

    #[ink(extension = 3511, returns_result = false, handle_status = false)]
    fn properties(
        collection_id: CollectionId,
        nft_id: Option<NftId>,
        key: Vec<u8>,
    ) -> Option<Vec<u8>>;

    #[ink(extension = 3512, returns_result = false, handle_status = false)]
    fn lock(collection_id: CollectionId, nft_id: NftId) -> bool;

    // WRITE functions
    #[ink(extension = 3513)]
    fn mint_nft(
        owner: AccountId,
        collection_id: u32,
        royalty_recipient: Option<AccountId>,
        royalty: Option<u32>,
        metadata: Vec<u8>,
        transferable: bool,
        resources: Option<Vec<ResourceTypes>>,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3514)]
    fn mint_nft_directly_to_nft(
        owner: (CollectionId, NftId),
        collection_id: u32,
        royalty_recipient: Option<AccountId>,
        royalty: Option<u32>,
        metadata: Vec<u8>,
        transferable: bool,
        resources: Option<Vec<ResourceTypes>>,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3515)]
    fn create_collection(
        metadata: Vec<u8>,
        max: Option<u32>,
        symbol: Vec<u8>,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3516)]
    fn burn_nft(
        collection_id: CollectionId,
        nft_id: NftId,
        max_burns: u32,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3517)]
    fn destroy_collection(collection_id: CollectionId) -> Result<(), RmrkError>;

    #[ink(extension = 3518)]
    fn send(
        collection_id: CollectionId,
        nft_id: NftId,
        new_owner: AccountIdOrCollectionNftTuple,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3519)]
    fn accept_nft(
        collection_id: CollectionId,
        nft_id: NftId,
        new_owner: AccountIdOrCollectionNftTuple,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3520)]
    fn reject_nft(collection_id: CollectionId, nft_id: NftId) -> Result<(), RmrkError>;

    #[ink(extension = 3521)]
    fn change_collection_issuer(
        collection_id: CollectionId,
        new_issuer: AccountId,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3522)]
    fn set_property(
        collection_id: CollectionId,
        maybe_nft_id: Option<NftId>,
        key: Vec<u8>,
        value: Vec<u8>,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3523)]
    fn lock_collection(collection_id: CollectionId) -> Result<(), RmrkError>;

    #[ink(extension = 3524)]
    fn add_basic_resource(
        collection_id: CollectionId,
        nft_id: NftId,
        resource: BasicResource,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3525)]
    fn add_composable_resource(
        collection_id: CollectionId,
        nft_id: NftId,
        resource: ComposableResource,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3526)]
    fn add_slot_resource(
        collection_id: CollectionId,
        nft_id: NftId,
        resource: SlotResource,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3527)]
    fn accept_resource(
        collection_id: CollectionId,
        nft_id: NftId,
        resource_id: ResourceId,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3528)]
    fn remove_resource(
        collection_id: CollectionId,
        nft_id: NftId,
        resource_id: ResourceId,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3529)]
    fn accept_resource_removal(
        collection_id: CollectionId,
        nft_id: NftId,
        resource_id: ResourceId,
    ) -> Result<(), RmrkError>;

    #[ink(extension = 3530)]
    fn set_priority(
        collection_id: CollectionId,
        nft_id: NftId,
        priorities: Vec<ResourceId>,
    ) -> Result<(), RmrkError>;
}

#[ink::contract(env = crate::CustomEnvironment)]
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
            self.env().extension().next_nft_id(collection_id)
        }

        #[ink(message)]
        pub fn collection_index(&self) -> CollectionId {
            self.env().extension().collection_index()
        }

        #[ink(message)]
        pub fn next_resource_id(&self, collection_id: CollectionId, nft_id: NftId) -> ResourceId {
            self.env()
                .extension()
                .next_resource_id(collection_id, nft_id)
        }

        #[ink(message)]
        pub fn collections(&self, collection_id: CollectionId) -> Option<CollectionInfo> {
            self.env().extension().collections(collection_id)
        }

        #[ink(message)]
        pub fn nfts(&self, collection_id: CollectionId, nft_id: NftId) -> Option<NftInfo> {
            self.env().extension().nfts(collection_id, nft_id)
        }

        #[ink(message)]
        pub fn priorities(
            &self,
            collection_id: CollectionId,
            nft_id: NftId,
            resource_id: ResourceId,
        ) -> Option<u32> {
            self.env()
                .extension()
                .priorities(collection_id, nft_id, resource_id)
        }

        #[ink(message)]
        pub fn children(
            &self,
            parent: (CollectionId, NftId),
            child: (CollectionId, NftId),
        ) -> Option<()> {
            self.env().extension().children(parent, child)
        }

        #[ink(message)]
        pub fn resources(
            &self,
            collection_id: CollectionId,
            nft_id: NftId,
            resource_id: ResourceId,
        ) -> Option<ResourceInfo> {
            self.env()
                .extension()
                .resources(collection_id, nft_id, resource_id)
        }

        #[ink(message)]
        pub fn equippable_bases(
            &self,
            collection_id: CollectionId,
            nft_id: NftId,
            base_id: BaseId,
        ) -> Option<()> {
            self.env()
                .extension()
                .equippable_bases(collection_id, nft_id, base_id)
        }

        #[ink(message)]
        pub fn equippable_slots(
            &self,
            collection_id: CollectionId,
            nft_id: NftId,
            resource_id: ResourceId,
            base_id: BaseId,
            slot_id: SlotId,
        ) -> Option<()> {
            self.env().extension().equippable_slots(
                collection_id,
                nft_id,
                resource_id,
                base_id,
                slot_id,
            )
        }

        #[ink(message)]
        pub fn properties(
            &self,
            collection_id: CollectionId,
            nft_id: Option<NftId>,
            key: Vec<u8>,
        ) -> Option<Vec<u8>> {
            self.env()
                .extension()
                .properties(collection_id, nft_id, key)
        }

        #[ink(message)]
        pub fn lock(&self, collection_id: CollectionId, nft_id: NftId) -> bool {
            self.env().extension().lock(collection_id, nft_id)
        }

        /// write functions
        #[ink(message)]
        pub fn mint_ntf(
            &mut self,
            owner: AccountId,
            collection_id: u32,
            royalty_recipient: Option<AccountId>,
            royalty: Option<u32>,
            metadata: Vec<u8>,
            transferable: bool,
            resources: Option<Vec<ResourceTypes>>,
        ) -> Result<(), RmrkError> {
            self.env().extension().mint_nft(
                owner,
                collection_id,
                royalty_recipient,
                royalty,
                metadata,
                transferable,
                resources,
            )
        }

        #[ink(message)]
        pub fn mint_ntf_directly_to_nft(
            &mut self,
            owner: (CollectionId, NftId),
            collection_id: u32,
            royalty_recipient: Option<AccountId>,
            royalty: Option<u32>,
            metadata: Vec<u8>,
            transferable: bool,
            resources: Option<Vec<ResourceTypes>>,
        ) -> Result<(), RmrkError> {
            self.env().extension().mint_nft_directly_to_nft(
                owner,
                collection_id,
                royalty_recipient,
                royalty,
                metadata,
                transferable,
                resources,
            )
        }

        #[ink(message)]
        pub fn create_collection(
            &mut self,
            metadata: Vec<u8>,
            max: Option<u32>,
            symbol: Vec<u8>,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .create_collection(metadata, max, symbol)
        }

        #[ink(message)]
        pub fn burn_nft(
            &mut self,
            collection_id: CollectionId,
            nft_id: NftId,
            max_burns: u32,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .burn_nft(collection_id, nft_id, max_burns)
        }

        #[ink(message)]
        pub fn destroy_collection(&mut self, collection_id: CollectionId) -> Result<(), RmrkError> {
            self.env().extension().destroy_collection(collection_id)
        }

        #[ink(message)]
        pub fn send(
            &mut self,
            collection_id: CollectionId,
            nft_id: NftId,
            new_owner: AccountIdOrCollectionNftTuple,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .send(collection_id, nft_id, new_owner)
        }

        #[ink(message)]
        pub fn accept_nft(
            &mut self,
            collection_id: CollectionId,
            nft_id: NftId,
            new_owner: AccountIdOrCollectionNftTuple,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .accept_nft(collection_id, nft_id, new_owner)
        }

        #[ink(message)]
        pub fn reject_nft(
            &mut self,
            collection_id: CollectionId,
            nft_id: NftId,
        ) -> Result<(), RmrkError> {
            self.env().extension().reject_nft(collection_id, nft_id)
        }

        #[ink(message)]
        pub fn change_collection_issuer(
            &mut self,
            collection_id: CollectionId,
            new_issuer: AccountId,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .change_collection_issuer(collection_id, new_issuer)
        }

        #[ink(message)]
        pub fn set_property(
            &mut self,
            collection_id: CollectionId,
            maybe_nft_id: Option<NftId>,
            key: Vec<u8>,
            value: Vec<u8>,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .set_property(collection_id, maybe_nft_id, key, value)
        }

        #[ink(message)]
        pub fn lock_collection(&mut self, collection_id: CollectionId) -> Result<(), RmrkError> {
            self.env().extension().lock_collection(collection_id)
        }

        #[ink(message)]
        pub fn add_basic_resource(
            &mut self,
            collection_id: CollectionId,
            nft_id: NftId,
            resource: BasicResource,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .add_basic_resource(collection_id, nft_id, resource)
        }

        #[ink(message)]
        pub fn add_composable_resource(
            &mut self,
            collection_id: CollectionId,
            nft_id: NftId,
            resource: ComposableResource,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .add_composable_resource(collection_id, nft_id, resource)
        }

        #[ink(message)]
        pub fn add_slot_resource(
            &mut self,
            collection_id: CollectionId,
            nft_id: NftId,
            resource: SlotResource,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .add_slot_resource(collection_id, nft_id, resource)
        }

        #[ink(message)]
        pub fn accept_resource(
            &mut self,
            collection_id: CollectionId,
            nft_id: NftId,
            resource_id: ResourceId,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .accept_resource(collection_id, nft_id, resource_id)
        }

        #[ink(message)]
        pub fn remove_resource(
            &mut self,
            collection_id: CollectionId,
            nft_id: NftId,
            resource_id: ResourceId,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .remove_resource(collection_id, nft_id, resource_id)
        }

        #[ink(message)]
        pub fn accept_resource_removal(
            &mut self,
            collection_id: CollectionId,
            nft_id: NftId,
            resource_id: ResourceId,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .accept_resource_removal(collection_id, nft_id, resource_id)
        }

        #[ink(message)]
        pub fn set_priority(
            &mut self,
            collection_id: CollectionId,
            nft_id: NftId,
            priorities: Vec<ResourceId>,
        ) -> Result<(), RmrkError> {
            self.env()
                .extension()
                .set_priority(collection_id, nft_id, priorities)
        }
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
