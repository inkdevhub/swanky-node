use ink_env::AccountId;
use ink_prelude::vec::Vec;
use scale::{Decode, Encode};

pub type CollectionId = u32;
pub type NftId = u32;
pub type ResourceId = u32;
pub type BaseId = u32;
pub type SlotId = u32;
pub type PartId = u32;

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct CollectionInfo {
    /// Current bidder and bid price.
    pub issuer: AccountId,

    pub metadata: Vec<u8>,
    pub max: Option<u32>,

    pub symbol: Vec<u8>,
    pub nfts_count: u32,
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct NftInfo {
    /// The owner of the NFT, can be either an Account or a tuple (CollectionId, NftId)
    pub owner: AccountIdOrCollectionNftTuple,
    /// Royalty (optional)
    pub royalty: Option<RoyaltyInfo>,

    /// Arbitrary data about an instance, e.g. IPFS hash
    pub metadata: Vec<u8>,

    /// Equipped state
    pub equipped: bool,
    /// Pending state (if sent to NFT)
    pub pending: bool,
    /// transferability ( non-transferable is "souldbound" )
    pub transferable: bool,
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AccountIdOrCollectionNftTuple {
    AccountId(AccountId),
    CollectionAndNftTuple(CollectionId, NftId),
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct RoyaltyInfo {
    /// Recipient (AccountId) of the royalty
    pub recipient: AccountId,
    /// Amount (Permill) of the royalty
    pub amount: u32,
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ResourceInfo {
    pub id: ResourceId,

    resources: ResourceTypes,

    pub pending: bool,
    pub pending_removal: bool,
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ResourceTypes {
    Basic(BasicResource),
    Composable(ComposableResource),
    Slot(SlotResource),
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct BasicResource {
    /// If the resource is Media, the base property is absent. Media src should be a URI like an
    /// IPFS hash.
    pub src: Option<Vec<u8>>,

    /// Reference to IPFS location of metadata
    pub metadata: Option<Vec<u8>>,

    /// Optional location or identier of license
    pub license: Option<Vec<u8>>,

    /// If the resource has the thumb property, this will be a URI to a thumbnail of the given
    /// resource. For example, if we have a composable NFT like a Kanaria bird, the resource is
    /// complex and too detailed to show in a search-results page or a list. Also, if a bird owns
    /// another bird, showing the full render of one bird inside the other's inventory might be a
    /// bit of a strain on the browser. For this reason, the thumb value can contain a URI to an
    /// image that is lighter and faster to load but representative of this resource.
    pub thumb: Option<Vec<u8>>,
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ComposableResource {
    /// If a resource is composed, it will have an array of parts that compose it
    pub parts: Vec<PartId>,

    /// A Base is uniquely identified by the combination of the word `base`, its minting block
    /// number, and user provided symbol during Base creation, glued by dashes `-`, e.g.
    /// base-4477293-kanaria_superbird.
    pub base: BaseId,

    /// If the resource is Media, the base property is absent. Media src should be a URI like an
    /// IPFS hash.
    pub src: Option<Vec<u8>>,

    /// Reference to IPFS location of metadata
    pub metadata: Option<Vec<u8>>,

    /// If the resource has the slot property, it was designed to fit into a specific Base's slot.
    pub slot: Option<(BaseId, SlotId)>,

    /// If the resource has the slot property, it was designed to fit into a specific Base's slot.
    /// The baseslot will be composed of two dot-delimited values, like so:
    /// "base-4477293-kanaria_superbird.machine_gun_scope". This means: "This resource is
    /// compatible with the machine_gun_scope slot of base base-4477293-kanaria_superbird

    /// Optional location or identier of license
    pub license: Option<Vec<u8>>,

    /// If the resource has the thumb property, this will be a URI to a thumbnail of the given
    /// resource. For example, if we have a composable NFT like a Kanaria bird, the resource is
    /// complex and too detailed to show in a search-results page or a list. Also, if a bird owns
    /// another bird, showing the full render of one bird inside the other's inventory might be a
    /// bit of a strain on the browser. For this reason, the thumb value can contain a URI to an
    /// image that is lighter and faster to load but representative of this resource.
    pub thumb: Option<Vec<u8>>,
}

#[derive(PartialEq, Debug, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct SlotResource {
    /// A Base is uniquely identified by the combination of the word `base`, its minting block
    /// number, and user provided symbol during Base creation, glued by dashes `-`, e.g.
    /// base-4477293-kanaria_superbird.
    pub base: BaseId,

    /// If the resource is Media, the base property is absent. Media src should be a URI like an
    /// IPFS hash.
    pub src: Option<Vec<u8>>,

    /// Reference to IPFS location of metadata
    pub metadata: Option<Vec<u8>>,

    /// If the resource has the slot property, it was designed to fit into a specific Base's slot.
    /// The baseslot will be composed of two dot-delimited values, like so:
    /// "base-4477293-kanaria_superbird.machine_gun_scope". This means: "This resource is
    /// compatible with the machine_gun_scope slot of base base-4477293-kanaria_superbird
    pub slot: SlotId,

    /// The license field, if present, should contain a link to a license (IPFS or static HTTP
    /// url), or an identifier, like RMRK_nocopy or ipfs://ipfs/someHashOfLicense.
    pub license: Option<Vec<u8>>,

    /// If the resource has the thumb property, this will be a URI to a thumbnail of the given
    /// resource. For example, if we have a composable NFT like a Kanaria bird, the resource is
    /// complex and too detailed to show in a search-results page or a list. Also, if a bird owns
    /// another bird, showing the full render of one bird inside the other's inventory might be a
    /// bit of a strain on the browser. For this reason, the thumb value can contain a URI to an
    /// image that is lighter and faster to load but representative of this resource.
    pub thumb: Option<Vec<u8>>,
}
