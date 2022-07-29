use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::{LookupMap, Vector},
    env,
    json_types::U128,
    near_bindgen, require, AccountId,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    map: LookupMap<AccountId, Option<String>>,
    keys: Vector<AccountId>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            map: LookupMap::new(b"m"),
            keys: Vector::new(b"v"),
        }
    }
}

/// Starting index, must be smaller than total number of entries.
/// If not filled in, defaults to 0
/// @pattern ^[0-9]+$
#[witgen::witgen]
pub type Start = String;

#[witgen::witgen]
/// Limit how many recommendations to get after start
pub type Limit = u64;

#[near_bindgen]
impl Contract {
    /// ㅤ                                                                                    .
    /// This Contract was built with raen. Each item on the left is a contract method you can call.
    /// To learn more checkout the guide: https://raen.dev/guide
    /// ㅤ                                                                                    .
    /// Do you support the NEAR foundation funding continued work on RAEN?
    /// ㅤ                                                                                    .
    /// If you do, please sign here and if you want leave a message!
    /// ㅤ                                                                                    .
    /// Signing and leaving a message has a storage cost, you will be refunded what isn't required.
    /// Suggestion for attached deposit 100000000000000000000000 yn = 0.1 N. Likely this is filled in already.
    /// ㅤ                                                                                    .
    /// 
    #[payable]
    pub fn recommend(&mut self, message: Option<String>) {
        let signer = env::signer_account_id();
        let before = env::storage_usage();
        if self.map.insert(&signer, &message).is_none() {
            self.keys.push(&signer);
        }
        let after = env::storage_usage() - before;
        let cost = after as u128 * env::STORAGE_PRICE_PER_BYTE;
        let attached_deposit = env::attached_deposit();
        require!(cost <= attached_deposit, &format!("Required {cost} yN"));
        let left_over_funds = attached_deposit - cost;
        if left_over_funds > 0 {
            env::promise_batch_action_transfer(
                env::promise_batch_create(&env::predecessor_account_id()),
                left_over_funds,
            );
        }
        env::log_str(&format!("Thank you, {signer}!"));
    }

    /// Get a range of recommendations
    pub fn get_recommendations(
        &self,
        start: Option<Start>,
        limit: Option<Limit>,
    ) -> HashMap<AccountId, Option<String>> {
        let len = self.keys.len();
        let start = start.map(|start| u128::from_str_radix(&start, 10).expect("opps bad u128") as u64).unwrap_or_default();
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
