#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use sp_runtime::{DispatchError, ModuleError};

pub enum RmrkFunc {
	// getters
	CollectionIndex,
	Collections,
	Nfts,
	Priorities,
	Children,
	Resources,
	EquippableBases,
	EquippableSlots,
	Properties,
	Lock,

	// extrinsics
	MintNft,
	MintNftDirectlyToNft,
	CreateCollection,
	BurnNft,
	DestroyCollection,
	Send,
	AcceptNft,
	RejectNft,
	ChangeCollectionIssuer,
	SetProperty,
	LockCollection,
	AddBasicResource,
	AddComposableResource,
	AddSlotResource,
	AcceptResource,
	RemoveResource,
	AcceptResourceRemoval,
	SetPriority,
}

impl TryFrom<u32> for RmrkFunc {
	type Error = DispatchError;

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		return match value {
			// getters
			0x0001 => Ok(RmrkFunc::CollectionIndex),
			0x0002 => Ok(RmrkFunc::Collections),
			0x0003 => Ok(RmrkFunc::Nfts),
			0x0004 => Ok(RmrkFunc::Priorities),
			0x0005 => Ok(RmrkFunc::Children),
			0x0006 => Ok(RmrkFunc::Resources),
			0x0007 => Ok(RmrkFunc::EquippableBases),
			0x0008 => Ok(RmrkFunc::EquippableSlots),
			0x0009 => Ok(RmrkFunc::Properties),
			0x000A => Ok(RmrkFunc::Lock),

			// extrinsics
			0x1001 => Ok(RmrkFunc::MintNft),
			0x1002 => Ok(RmrkFunc::MintNftDirectlyToNft),
			0x1003 => Ok(RmrkFunc::CreateCollection),
			0x1004 => Ok(RmrkFunc::BurnNft),
			0x1005 => Ok(RmrkFunc::DestroyCollection),
			0x1006 => Ok(RmrkFunc::Send),
			0x1007 => Ok(RmrkFunc::AcceptNft),
			0x1008 => Ok(RmrkFunc::RejectNft),
			0x1009 => Ok(RmrkFunc::ChangeCollectionIssuer),
			0x100A => Ok(RmrkFunc::SetProperty),
			0x100B => Ok(RmrkFunc::LockCollection),
			0x100C => Ok(RmrkFunc::AddBasicResource),
			0x100D => Ok(RmrkFunc::AddComposableResource),
			0x100E => Ok(RmrkFunc::AddSlotResource),
			0x100F => Ok(RmrkFunc::AcceptResource),
			0x1010 => Ok(RmrkFunc::RemoveResource),
			0x1011 => Ok(RmrkFunc::AcceptResourceRemoval),
			0x1012 => Ok(RmrkFunc::SetPriority),
			_ => Err(DispatchError::Other("RmrkExtension: Unimplemented func_id")),
		}
	}
}

#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
pub enum RmrkError {
	/// Error names should be descriptive.
	None,
	/// Errors should have helpful documentation associated with them.
	StorageOverflow,
	TooLong,
	NoAvailableCollectionId,
	NoAvailableResourceId,
	MetadataNotSet,
	RecipientNotSet,
	NoAvailableNftId,
	NotInRange,
	RoyaltyNotSet,
	CollectionUnknown,
	NoPermission,
	NoWitness,
	CollectionNotEmpty,
	CollectionFullOrLocked,
	CannotSendToDescendentOrSelf,
	ResourceAlreadyExists,
	EmptyResource,
	TooManyRecursions,
	NftIsLocked,
	CannotAcceptNonOwnedNft,
	CannotRejectNonOwnedNft,
	CannotRejectNonPendingNft,
	ResourceDoesntExist,
	/// Accepting a resource that is not pending should fail
	ResourceNotPending,
	NonTransferable,
	// Must unequip an item before sending (this only applies to the
	// rmrk-equip pallet but the send operation lives in rmrk-core)
	CannotSendEquippedItem,
}

impl TryFrom<DispatchError> for RmrkError {
	type Error = DispatchError;

	fn try_from(input: DispatchError) -> Result<Self, Self::Error> {
		let error_text = match input {
			DispatchError::Module(ModuleError { message, .. }) => message,
			_ => Some("No module error Info"),
		};
		match error_text {
			Some("NoneValue") => Ok(RmrkError::None),
			Some("StorageOverflow") => Ok(RmrkError::StorageOverflow),
			Some("TooLong") => Ok(RmrkError::TooLong),
			Some("NoAvailableCollectionId") => Ok(RmrkError::NoAvailableCollectionId),
			Some("NoAvailableResourceId") => Ok(RmrkError::NoAvailableResourceId),
			Some("MetadataNotSet") => Ok(RmrkError::MetadataNotSet),
			Some("RecipientNotSet") => Ok(RmrkError::RecipientNotSet),
			Some("NoAvailableNftId") => Ok(RmrkError::NoAvailableNftId),
			Some("NotInRange") => Ok(RmrkError::NotInRange),
			Some("RoyaltyNotSet") => Ok(RmrkError::RoyaltyNotSet),
			Some("CollectionUnknown") => Ok(RmrkError::CollectionUnknown),
			Some("NoPermission") => Ok(RmrkError::NoPermission),
			Some("NoWitness") => Ok(RmrkError::NoWitness),
			Some("CollectionNotEmpty") => Ok(RmrkError::CollectionNotEmpty),
			Some("CollectionFullOrLocked") => Ok(RmrkError::CollectionFullOrLocked),
			Some("CannotSendToDescendentOrSelf") => Ok(RmrkError::CannotSendToDescendentOrSelf),
			Some("ResourceAlreadyExists") => Ok(RmrkError::ResourceAlreadyExists),
			Some("EmptyResource") => Ok(RmrkError::EmptyResource),
			Some("TooManyRecursions") => Ok(RmrkError::TooManyRecursions),
			Some("NftIsLocked") => Ok(RmrkError::NftIsLocked),
			Some("CannotAcceptNonOwnedNft") => Ok(RmrkError::CannotAcceptNonOwnedNft),
			Some("CannotRejectNonOwnedNft") => Ok(RmrkError::CannotRejectNonOwnedNft),
			Some("CannotRejectNonPendingNft") => Ok(RmrkError::CannotRejectNonPendingNft),
			Some("ResourceDoesntExist") => Ok(RmrkError::ResourceDoesntExist),
			Some("ResourceNotPending") => Ok(RmrkError::ResourceNotPending),
			Some("NonTransferable") => Ok(RmrkError::NonTransferable),
			Some("CannotSendEquippedItem") => Ok(RmrkError::CannotSendEquippedItem),
			_ => Err(DispatchError::Other("RmrkExtension: Unknown error")),
		}
	}
}
