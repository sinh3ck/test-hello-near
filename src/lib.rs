// Find all our documentation at https://docs.near.org
use near_sdk::{log, near};
use near_contract_standards::fungible_token::{
    metadata::{FungibleTokenMetadata, FT_METADATA_SPEC}};
use near_sdk::collections::LazyOption;

fn ft_metadata_default() -> FungibleTokenMetadata {
    FungibleTokenMetadata {
        spec: FT_METADATA_SPEC.to_string(),
        name: "NEAR Token".to_string(),
        symbol: "NEART".to_string(),
        icon: Some(r#""#.into()),
        reference: Some("".into()), 
        reference_hash: None,
        decimals: 24,
    }
}

fn ft_metadata_init_lazy_container() -> LazyOption<FungibleTokenMetadata> {
    let metadata: LazyOption<FungibleTokenMetadata>;
    metadata = LazyOption::new(b"neart".to_vec(), None);
    return metadata;
}

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    greeting: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            greeting: "Hello".to_string(),
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, greeting: String) {
        log!("Saving greeting: {greeting}");
        self.greeting = greeting;
    }

    pub fn ft_metadata_set(&self, data: FungibleTokenMetadata) {
        //self.assert_owner_calling();
        let mut metadata = ft_metadata_init_lazy_container();
        metadata.set(&data); //save into storage
    }

    pub fn ft_metadata(&self) -> FungibleTokenMetadata {
        let metadata = ft_metadata_init_lazy_container();
        //load from storage or return default
        return metadata.get().unwrap_or(ft_metadata_default());
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello");
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy");
    }

    #[test]
    fn set_metadata_set() {
        let mut contract = Contract::default();
        let mut metadata = contract.ft_metadata();
        println!("metadata: {:#?}", metadata.name);

        metadata.name = "YoYo".to_string();
        contract.ft_metadata_set(metadata);

        let mut metadata2 = contract.ft_metadata();
        println!("metadata2: {:#?}", metadata2.name);
    }
}
