use multiversx_sc_scenario::imports::*;
use nft_staking::reward::reward_rate::REWARD_RATE_DENOMINATION;

pub const SC_ADDRESS: TestSCAddress = TestSCAddress::new("sc");
pub const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
pub const USER_ADDRESS: TestAddress = TestAddress::new("user");

pub const NFT_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("NFT-123456");
pub const SFT_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("NFT-789012");
pub const REWARD_TOKEN_ID_1: TestTokenIdentifier = TestTokenIdentifier::new("REWARD1-123456");
pub const REWARD_TOKEN_ID_2: TestTokenIdentifier = TestTokenIdentifier::new("REWARD2-123456");
pub const UNSUPPORTED_NFT_TOKEN_ID: TestTokenIdentifier =
    TestTokenIdentifier::new("UNSUPPORTED-NFT");

pub const NFTSFT_NONCES: [u64; 5] = [1, 2, 3, 4, 5];
pub const INITIAL_SFT_BALANCE: u64 = 10;
pub const INITIAL_ESDT_BALANCE: u64 = 1_000_000;

pub const CODE_PATH: MxscPath = MxscPath::new("output/nft-staking.mxsc.json");
