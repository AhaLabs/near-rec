//! This contract implements simple counter backed by storage on blockchain.
//!
//! The contract provides methods to [increment] / [decrement] counter and
//! get it's current value [get_num] or [reset].
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::{UnorderedMap, Vector},
    env,
    json_types::U128,
    near_bindgen, require, AccountId,
};
use std::collections::HashMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    map: UnorderedMap<AccountId, Option<String>>,
    keys: Vector<AccountId>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            map: UnorderedMap::new(b"m"),
            keys: Vector::new(b"v"),
        }
    }
}

#[witgen::witgen]
pub type Start = Option<U128>;

#[witgen::witgen]
pub type Limit = Option<u64>;

#[near_bindgen]
impl Contract {
    /// Do you recommend that the NEAR foundation should fund work on RAEN?
    /// If you do, please sign here and leave a message!
    pub fn recommend(&mut self, message: Option<String>) {
        let signer = env::signer_account_id();
        if self.map.insert(&signer, &message).is_none() {
            self.keys.push(&signer);
            env::log_str(&format!("Thank you, {signer}!"));
        }
    }

    /// Get a range of recommendations
    /// Starting index, must be smaller than total number of entries.
    /// If not filled in, defaults to 0
    /// limit is the number of elements after `start`, default is len - start, which is also maximimum.
    pub fn get_recommendations(
        &self,
        start: Start,
        limit: Limit,
    ) -> HashMap<AccountId, Option<String>> {
        let len = self.keys.len();
        let start = start.map(|start| start.0 as u64).unwrap_or_default();
        require!(start < len, "start must be less than len");
        let end = u64::min(start + (limit.unwrap_or(len - start)), len);
        (start..end)
            .map(|i| self.keys.get(i).unwrap())
            .map(|key| (key.clone(), self.map.get(&key).unwrap()))
            .collect()
    }

    /// Total number of recommendations received
    pub fn total_recommendations(&self) -> u64 {
        self.keys.len()
    }

    /// Get the recommendation from a specific account
    pub fn get_recommendation(&self, account_id: AccountId) -> Option<Option<String>> {
        self.map.get(&account_id)
    }
}
