// errors
pub const ERR_NFT_COLLECTION_NOT_ALLOWED: &str = "NFT collection not allowed";
pub const ERR_USER_HAS_NOT_ENOUGH_STAKED_BALANCE: &str = "User has not enough staked balance";
pub const ERR_STAKING_DISABLED: &str = "Staking is disabled";
pub const ERR_NO_UNSTAKED_ITEMS: &str = "No unstaked items";

// default configuration
pub const UNSTAKE_PENALTY: u64 = 7 * 24 * 3600u64; // 7 days
pub const DEFAULT_NFT_SCORE: u64 = 1_000_000; // 1000
