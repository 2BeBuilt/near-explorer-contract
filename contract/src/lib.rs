use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractData {
    pub cid: String,
    pub lang: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    contracts: UnorderedMap<AccountId, ContractData>,
    secret: String,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(secret: String) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            contracts: UnorderedMap::new(b"c".to_vec()),
            secret: secret,
        }
    }

    pub fn set_contract(&mut self, cid: String, lang: String) {
        self.contracts.insert(&env::predecessor_account_id(), &ContractData {
            cid,
            lang,
        });
    }

    pub fn get_contract(&self, contract_id: AccountId) -> Option<ContractData> {        
        return self.contracts.get(&contract_id);
    }

    pub fn get_contracts(&self) -> Vec<(AccountId, ContractData)> {
        return self.contracts.to_vec();
    }
}
