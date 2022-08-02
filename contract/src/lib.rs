use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::{LookupMap, Vector},
    env, near_bindgen, require,
    serde::{Deserialize, Serialize},
    AccountId,
};

/// Optional fields to provide more info
#[witgen::witgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserInfo {
    /// Name you go by:
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// Which company or project are you affliated with:
    #[serde(skip_serializing_if = "Option::is_none")]
    affiliation: Option<String>,
    /// Message of support
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    map: LookupMap<AccountId, UserInfo>,
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

/// Starting index, must be smaller than total number of entries.ㅤ                                                                                    .
/// If not filled in, defaults to 0
/// @pattern ^[0-9]+$
#[witgen::witgen]
pub type Start = String;

#[witgen::witgen]
/// Limit how many recommendations to get after start
pub type Limit = u64;

#[near_bindgen]
impl Contract {
    /// ㅤ                                                                                    
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
    pub fn recommend(&mut self, user_info: &UserInfo) {
        let signer = env::signer_account_id();
        let before = env::storage_usage();
        if self.map.insert(&signer, user_info).is_none() {
            self.keys.push(&signer);
        }
        let after = env::storage_usage() - before;
        let cost = u128::from(after) * env::STORAGE_PRICE_PER_BYTE;
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

    /// ㅤ                                                                                    .
    /// This Contract was built with RAEN. Each item on the left is a contract method you can call.
    /// To learn more check out the guide: https://raen.dev
    /// ㅤ                                                                                    .
    /// This method gets a range of recommendations.
    /// 
    /// ㅤ                                                                                    .
    /// How many recommendations to return.ㅤ                                          .
    /// Default if not filled in is all after start.
    pub fn get_recommendations(
        &self,
        start: Option<Start>,
        limit: Option<Limit>,
    ) -> HashMap<AccountId, UserInfo> {
        let len = self.keys.len();
        let start = start
        .map(|start| start.parse::<u128>().expect("opps bad u128") as u64)
        .unwrap_or_default();
        if len == 0 {
          return HashMap::default();
        }
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
    pub fn get_recommendation(&self, account_id: &AccountId) -> Option<UserInfo> {
        self.map.get(account_id)
    }
}
