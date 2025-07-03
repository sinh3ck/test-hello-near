use serde_json::json;
use near_contract_standards::fungible_token::{
    metadata::{FungibleTokenMetadata}};

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let contract_wasm = near_workspaces::compile_project("./").await?;

    test_basics_on(&contract_wasm).await?;
    Ok(())
}

async fn test_basics_on(contract_wasm: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract = sandbox.dev_deploy(contract_wasm).await?;

    let user_account = sandbox.dev_create_account().await?;

    let outcome = user_account
        .call(contract.id(), "set_greeting")
        .args_json(json!({"greeting": "Hello World!"}))
        .transact()
        .await?;
    assert!(outcome.is_success());

    let user_message_outcome = contract.view("get_greeting").args_json(json!({})).await?;
    assert_eq!(user_message_outcome.json::<String>()?, "Hello World!");

    Ok(())
}

#[tokio::test]
async fn test_set_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let contract_wasm = near_workspaces::compile_project("./").await?;
    let sandbox = near_workspaces::sandbox().await?;
    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    let user_account = sandbox.dev_create_account().await?;

    let outcome = user_account
        .call(contract.id(), "ft_metadata")
        .view()
        .await?;
    //assert!(outcome.is_success());
    println!("metadata: {:#?}", outcome);

    let outcome = user_account
        .call(contract.id(), "ft_metadata_set")
        .args_json(json!(
            {"data":
                {
                    "spec":"ft-1.0.0",
                    "name":"Yoyo",
                    "symbol":"NEART",
                    "icon":"",
                    "reference":"",
                    "reference_hash":"",
                    "decimals":24
                }
            }))
        .transact()
        .await?;
    assert!(outcome.is_success());

    let outcome = user_account
        .call(contract.id(), "ft_metadata")
        .view()
        .await?;
    //assert!(outcome.is_success());
    println!("metadata: {:#?}", outcome.json::<FungibleTokenMetadata>()?.name);

    //let user_message_outcome = contract.view("get_greeting").args_json(json!({})).await?;
    //assert_eq!(user_message_outcome.json::<String>()?, "Hello World!");

    Ok(())
}


