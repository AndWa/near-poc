/*
 * POC smart contract for ETH ABI encoding/decoding and message passing
 *
 */

use ethabi::decode;
use ethabi::ParamType;
use ethabi::Token;
use k256::ecdsa::VerifyingKey;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Gas};
use primitive_types::H256;
use uint::hex;

mod ecrecover;
pub mod external;
mod tests;
pub use crate::external::*;

pub const TGAS: u64 = 1_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Gateway {}

impl Default for Gateway {
    fn default() -> Self {
        Self {}
    }
}

#[near_bindgen]
impl Gateway {
    pub fn sign_message(&self, message: String) -> String {
        let hash = ecrecover::hash_message(message);
        let full_hash = format!("{:#x}", hash);
        full_hash
    }

    pub fn abi_decode(&self, payload: String) -> Vec<String> {
        let payload_bytes = &hex::decode(&payload).unwrap();
        let result = decode(&[ParamType::String], &payload_bytes);
        assert_eq!(result.is_ok(), true);
        let values = result.unwrap();

        let result = values
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>();

        result
    }

    // TODO: Implement json structure for input instead of the message that will be able to map to the Tokens required for ABI encoding
    pub fn abi_encode(&self, message: String) -> String {
        let payload = ethabi::encode(&[Token::String(message.to_string())]);
        hex::encode(payload)
    }

    pub fn call_contract(
        &mut self,
        destination_chain: String,
        destination_contract_address: AccountId,
        payload: String,
    ) {
        contract_executable::ext(destination_contract_address.clone())
            .with_static_gas(Gas(5 * TGAS))
            .execute(
                "command_id".to_string(),
                destination_chain,
                env::predecessor_account_id(),
                payload,
            );
    }
}
