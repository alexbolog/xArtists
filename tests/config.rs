use multiversx_sc_scenario::imports::*;

pub const SC_ADDRESS: TestSCAddress = TestSCAddress::new("sc");
pub const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
pub const USER_ADDRESS: TestAddress = TestAddress::new("user");

pub const TRO_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("TRO");
pub const LP_TOKEN_ID_1: TestTokenIdentifier = TestTokenIdentifier::new("LPTOKEN1");
pub const LP_TOKEN_ID_2: TestTokenIdentifier = TestTokenIdentifier::new("LPTOKEN2");
pub const LP_TOKEN_ID_3: TestTokenIdentifier = TestTokenIdentifier::new("LPTOKEN3");
pub const UNSUPPORTED_LP_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("UNSUPPORTED_LP_TOKEN");

pub const INITIAL_TOKEN_BALANCE: u64 = 1_000_000;

pub const CODE_PATH: MxscPath = MxscPath::new("output/tro-staking.mxsc.json");