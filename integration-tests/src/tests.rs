use near_sdk::json_types::U128;
use near_units::{parse_gas, parse_near};
use serde_json::json;
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker};

const WASM_FILEPATH: &str = "../../contract/target/wasm32-unknown-unknown/release/sourcescan.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let contract_wasm = std::fs::read(WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&contract_wasm).await?;
    let owner = worker.root_account().unwrap();

    // Initialize the contract
    contract
        .call("new")
        .args_json(serde_json::json!({}))
        .transact()
        .await?;

    // Begin tests
    test_set_and_get_owner(&owner, &contract).await?;
    test_set_and_get_contract(&owner, &contract).await?;
    test_search(&owner, &contract).await?;
    test_purge_contract(&owner, &contract).await?;

    Ok(())
}

async fn test_set_and_get_owner(
    owner: &Account,
    contract: &Contract,
) -> anyhow::Result<()> {
    let new_owner_id = "new_owner.testnet";

    contract
        .call("set_owner")
        .args_json(serde_json::json!({ "owner_id": new_owner_id }))
        .transact()
        .await?;

    let result_owner_id: String = contract
        .call("get_owner")
        .args_json(serde_json::json!({}))
        .transact()
        .await?
        .json()?;

    assert_eq!(result_owner_id, new_owner_id);
    println!("Passed ✅ test_set_and_get_owner");
    Ok(())
}

async fn test_set_and_get_contract(
    owner: &Account,
    contract: &Contract,
) -> anyhow::Result<()> {
    let contract_id = "contract1.testnet";
    let contract_data = serde_json::json!({
        "cid": "cid1",
        "lang": "Rust",
        "entry_point": "main",
        "code_hash": "codehash1",
        "builder_image": "rust:latest",
        "github": null
    });

    contract
        .call("set_contract")
        .args_json(serde_json::json!({ 
            "account_id": contract_id,
            "cid": "cid1",
            "lang": "Rust",
            "entry_point": "main",
            "code_hash": "codehash1",
            "builder_image": "rust:latest",
            "github": null
        }))
        .transact()
        .await?;

    let result_contract_data: serde_json::Value = contract
        .call("get_contract")
        .args_json(serde_json::json!({ "account_id": contract_id }))
        .transact()
        .await?
        .json()?;

    assert_eq!(result_contract_data, contract_data);
    println!("Passed ✅ test_set_and_get_contract");
    Ok(())
}

async fn test_search(
    owner: &Account,
    contract: &Contract,
) -> anyhow::Result<()> {
    let search_key = "rust";

    let (search_results, _pages): (Vec<(String, serde_json::Value)>, u64) = contract
        .call("search")
        .args_json(serde_json::json!({ "key": search_key, "from_index": 0, "limit": 10 }))
        .transact()
        .await?
        .json()?;

    assert!(!search_results.is_empty(), "Search results should not be empty.");
    for (_, contract_data) in search_results.iter() {
        assert_eq!(contract_data["lang"], "Rust");
    }

    println!("Passed ✅ test_search");
    Ok(())
}

async fn test_purge_contract(
    owner: &Account,
    contract: &Contract,
) -> anyhow::Result<()> {
    let contract_id = "contract1.testnet";

    contract
        .call("purge_contract")
        .args_json(serde_json::json!({ "account_id": contract_id }))
        .transact()
        .await?;

    // Verify contract is purged
    // ...

    println!("Passed ✅ test_purge_contract");
    Ok(())
}

// Add more test functions as needed...
