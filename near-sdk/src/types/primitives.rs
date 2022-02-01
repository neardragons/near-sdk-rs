#[cfg(not(target_arch = "wasm32"))]
use near_primitives_core::hash::CryptoHash;

#[cfg(not(target_arch = "wasm32"))]
pub use near_primitives_core::runtime::fees::RuntimeFeesConfig;

//* Type aliases from near_primitives_core

/// Hash used by a struct implementing the Merkle tree.
#[cfg(not(target_arch = "wasm32"))]
pub type MerkleHash = CryptoHash;

#[crate::witgen]
/// Validator identifier in current group.
pub type ValidatorId = u64;
#[crate::witgen]
/// Mask which validators participated in multi sign.
pub type ValidatorMask = Vec<bool>;
#[crate::witgen]
/// StorageUsage is used to count the amount of storage used by a contract.
pub type StorageUsage = u64;
#[crate::witgen]
/// StorageUsageChange is used to count the storage usage within a single contract call.
pub type StorageUsageChange = i64;
#[crate::witgen]
/// Nonce for transactions.
pub type Nonce = u64;
#[crate::witgen]
/// Height of the block.
pub type BlockHeight = u64;
#[crate::witgen]
/// Height of the epoch.
pub type EpochHeight = u64;
#[crate::witgen]
/// Shard index, from 0 to NUM_SHARDS - 1.
pub type ShardId = u64;
#[crate::witgen]
/// Balance is a type for storing amounts of tokens, specified in yoctoNEAR.
pub type Balance = u128;

#[crate::witgen]
/// Number of blocks in current group.
pub type NumBlocks = u64;
#[crate::witgen]
/// Number of shards in current group.
pub type NumShards = u64;
#[crate::witgen]
/// Number of seats of validators (block producer or hidden ones) in current group (settlement).
pub type NumSeats = u64;
#[crate::witgen]
/// Block height delta that measures the difference between `BlockHeight`s.
pub type BlockHeightDelta = u64;

pub type GCCount = u64;

#[crate::witgen]
pub type PromiseId = Vec<usize>;

pub type ProtocolVersion = u32;
