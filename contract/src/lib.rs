use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GithubData {
    pub owner: String,
    pub repo: String,
    pub sha: String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractData {
    pub cid: String,
    pub lang: String,
    pub entry_point: String,
    pub deploy_tx: String,
    pub github: Option<GithubData>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    contracts: UnorderedMap<AccountId, ContractData>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            contracts: UnorderedMap::new(b"c".to_vec()),
        }
    }

    pub fn set_contract(&mut self, cid: String, lang: String, entry_point: String, deploy_tx: String, github: Option<GithubData>) {
        self.contracts.insert(&env::predecessor_account_id(), &ContractData {
            cid: cid,
            lang: lang,
            entry_point: entry_point,
            deploy_tx: deploy_tx,
            github: match github {
                Some(github_data) => Some(GithubData {
                    owner: github_data.owner.clone(),
                    repo: github_data.repo.clone(),
                    sha: github_data.sha.clone(),
                }),
                None => None,
            },
        });
    }

    pub fn search(&self, key: String, from_index: usize, limit: usize) -> (Vec<(AccountId, ContractData)>, u64) {
        let mut result: Vec<(AccountId, ContractData)> = Vec::new();

        for (k, v) in self.contracts.iter()
        {            
            if k.as_str().to_lowercase().replace(".testnet", "").replace(".near", "").contains(&key.to_lowercase()) {
                result.push((k, v));
            }
        }
        
        let pages: u64 = self.get_pages(result.len() as u64, limit as u64);
        let filtered: Vec<(AccountId, ContractData)> = result
        .into_iter()
        .skip(from_index)
        .take(limit)
        .collect();

        return (filtered, pages);
    }

    pub fn get_contract(&self, contract_id: AccountId) -> Option<ContractData> {       
        return self.contracts.get(&contract_id);
    }

    pub fn get_contracts(&self, from_index: usize, limit: usize) -> (Vec<(AccountId, ContractData)>, u64) {
        let filtered:Vec<(AccountId, ContractData)> = self.contracts
        .iter()
        .skip(from_index)
        .take(limit)
        .collect();

        let pages: u64 = self.get_pages(self.contracts.len(), limit as u64);

        return (filtered, pages);
    }

    fn get_pages (&self, len: u64, limit: u64) -> u64 {
        return (len + limit - 1) / limit;
    }
}
