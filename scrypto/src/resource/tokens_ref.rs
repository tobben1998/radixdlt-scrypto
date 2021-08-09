use sbor::{Decode, Describe, Encode};

use crate::kernel::*;
use crate::types::*;

/// A borrowed reference to a `Tokens` bucket.
#[derive(Debug, Describe, Encode, Decode)]
pub struct TokensRef {
    bid: BID,
}

impl From<BID> for TokensRef {
    fn from(bid: BID) -> Self {
        Self { bid }
    }
}

impl TokensRef {
    pub fn amount(&self) -> U256 {
        let input = GetTokensAmountInput { tokens: self.bid };
        let output: GetTokensAmountOutput = call_kernel(GET_TOKENS_AMOUNT, input);

        output.amount
    }

    pub fn resource(&self) -> Address {
        let input = GetTokensResourceInput { tokens: self.bid };
        let output: GetTokensResourceOutput = call_kernel(GET_TOKENS_RESOURCE, input);

        output.resource
    }

    pub fn destroy(self) {
        let input = ReturnTokensInput {
            reference: self.bid,
        };
        let _: ReturnTokensOutput = call_kernel(RETURN_TOKENS, input);
    }
}