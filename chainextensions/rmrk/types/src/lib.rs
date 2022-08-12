#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::DispatchError;

pub enum RmrkFunc {
	// getters
	NextNftId,
	CollectionIndex,
	NextResourceId,
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
			0x0001 => Ok(RmrkFunc::NextNftId),
			0x0002 => Ok(RmrkFunc::CollectionIndex),
			0x0003 => Ok(RmrkFunc::NextResourceId),
			0x0004 => Ok(RmrkFunc::Collections),
			0x0005 => Ok(RmrkFunc::Nfts),
			0x0006 => Ok(RmrkFunc::Priorities),
			0x0007 => Ok(RmrkFunc::Children),
			0x0008 => Ok(RmrkFunc::Resources),
			0x0009 => Ok(RmrkFunc::EquippableBases),
			0x000A => Ok(RmrkFunc::EquippableSlots),
			0x000B => Ok(RmrkFunc::Properties),
			0x000C => Ok(RmrkFunc::Lock),

			// extrinsics
			0x000D => Ok(RmrkFunc::MintNft),
			0x000E => Ok(RmrkFunc::MintNftDirectlyToNft),
			0x000F => Ok(RmrkFunc::CreateCollection),
			0x0010 => Ok(RmrkFunc::BurnNft),
			0x0011 => Ok(RmrkFunc::DestroyCollection),
			0x0012 => Ok(RmrkFunc::Send),
			0x0013 => Ok(RmrkFunc::AcceptNft),
			0x0014 => Ok(RmrkFunc::RejectNft),
			0x0015 => Ok(RmrkFunc::ChangeCollectionIssuer),
			0x0016 => Ok(RmrkFunc::SetProperty),
			0x0017 => Ok(RmrkFunc::LockCollection),
			0x0018 => Ok(RmrkFunc::AddBasicResource),
			0x0019 => Ok(RmrkFunc::AddComposableResource),
			0x001A => Ok(RmrkFunc::AddSlotResource),
			0x001B => Ok(RmrkFunc::AcceptResource),
			0x001C => Ok(RmrkFunc::RemoveResource),
			0x001D => Ok(RmrkFunc::AcceptResourceRemoval),
			0x001E => Ok(RmrkFunc::SetPriority),
			_ => Err(DispatchError::Other("RmrkExtension: Unimplemented func_id")),
		}
	}
}
