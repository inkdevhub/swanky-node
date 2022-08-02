#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::DispatchError;

pub enum RmrkFunc {
	// getters
	NextNftId = 1,
	CollectionIndex = 2,
	NextResourceId = 3,
	Collections = 4,
	Nfts = 5,
	Priorities = 6,
	Children = 7,
	Resources = 8,
	EquippableBases = 9,
	EquippableSlots = 10,
	Properties = 11,
	Lock = 12,

	// extrinsics
	MintNft = 13,
	MintNftDirectlyToNft = 14,
	CreateCollection = 15,
	BurnNft = 16,
	DestroyCollection = 17,
	Send = 18,
	AcceptNft = 19,
	RejectNft = 20,
	ChangeCollectionIssuer = 21,
	SetProperty = 22,
	LockCollection = 23,
	AddBasicResource = 24,
	AddComposableResource = 25,
	AddSlotResource = 26,
	AcceptResource = 27,
	RemoveResource = 28,
	AcceptResourceRemoval = 29,
	SetPriority = 30,
}

impl TryFrom<u32> for RmrkFunc {
	type Error = DispatchError;

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		return match value {
			// getters
			1 => Ok(RmrkFunc::NextNftId),
			2 => Ok(RmrkFunc::CollectionIndex),
			3 => Ok(RmrkFunc::NextResourceId),
			4 => Ok(RmrkFunc::Collections),
			5 => Ok(RmrkFunc::Nfts),
			6 => Ok(RmrkFunc::Priorities),
			7 => Ok(RmrkFunc::Children),
			8 => Ok(RmrkFunc::Resources),
			9 => Ok(RmrkFunc::EquippableBases),
			10 => Ok(RmrkFunc::EquippableSlots),
			11 => Ok(RmrkFunc::Properties),
			12 => Ok(RmrkFunc::Lock),

			// extrinsics
			13 => Ok(RmrkFunc::MintNft),
			14 => Ok(RmrkFunc::MintNftDirectlyToNft),
			15 => Ok(RmrkFunc::CreateCollection),
			16 => Ok(RmrkFunc::BurnNft),
			17 => Ok(RmrkFunc::DestroyCollection),
			18 => Ok(RmrkFunc::Send),
			19 => Ok(RmrkFunc::AcceptNft),
			20 => Ok(RmrkFunc::RejectNft),
			21 => Ok(RmrkFunc::ChangeCollectionIssuer),
			22 => Ok(RmrkFunc::SetProperty),
			23 => Ok(RmrkFunc::LockCollection),
			24 => Ok(RmrkFunc::AddBasicResource),
			25 => Ok(RmrkFunc::AddComposableResource),
			26 => Ok(RmrkFunc::AddSlotResource),
			27 => Ok(RmrkFunc::AcceptResource),
			28 => Ok(RmrkFunc::RemoveResource),
			29 => Ok(RmrkFunc::AcceptResourceRemoval),
			30 => Ok(RmrkFunc::SetPriority),
			_ => Err(DispatchError::Other("RmrkExtension: Unimplemented func_id")),
		}
	}
}
