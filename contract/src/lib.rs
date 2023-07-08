use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

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
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            contracts: UnorderedMap::new(b"c".to_vec()),
            secret: "pLPov1SxdDXspEaBMQcb41Ay5lyjmlRX".to_string(),
        }
    }

    pub fn set_contract(&mut self, encrypted_cid: String, lang: String) {
        let mc: magic_crypt::MagicCrypt256 = new_magic_crypt!(&self.secret, 256);
        let cid: String = mc.decrypt_base64_to_string(&encrypted_cid).unwrap();

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
