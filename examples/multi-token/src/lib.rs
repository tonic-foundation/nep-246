use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption};
use near_sdk::json_types::U128;
use near_sdk::Promise;
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, BorshStorageKey, PanicOnDefault, PromiseOrValue,
};
use nep_246::multi_token::metadata::MT_METADATA_SPEC;
use nep_246::multi_token::token::{Approval, Token, TokenId};
use nep_246::multi_token::{
    core::MultiToken,
    metadata::{MtContractMetadata, TokenMetadata},
};
use std::collections::HashMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: MultiToken,
    metadata: LazyOption<MtContractMetadata>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    MultiToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        let metadata = MtContractMetadata {
            spec: MT_METADATA_SPEC.to_string(),
            name: "Test".to_string(),
            symbol: "OMG".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        };

        Self::new(owner_id, metadata)
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: MtContractMetadata) -> Self {
        require!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();

        Self {
            tokens: MultiToken::new(
                StorageKey::MultiToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    #[payable]
    pub fn mt_mint(
        &mut self,
        token_owner_id: AccountId,
        token_metadata: TokenMetadata,
        amount: Balance,
    ) -> Token {
        // Only the owner of the MFT contract can perform this operation
        assert_eq!(env::predecessor_account_id(), self.tokens.owner_id, "Unauthorized: {} != {}", env::predecessor_account_id(), self.tokens.owner_id);
        self.tokens.internal_mint(token_owner_id, Some(amount), Some(token_metadata), None)
    }

    pub fn register(&mut self, token_id: TokenId, account_id: AccountId) {
        self.tokens.internal_register_account(&token_id, &account_id)
    }
}

nep_246::impl_multi_token_core!(Contract, tokens);
nep_246::impl_multi_token_approval!(Contract, tokens);
nep_246::impl_multi_token_enumeration!(Contract, tokens);


#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};
    use super::*;

    fn create_token_md(title: String, description: String) -> TokenMetadata {
        TokenMetadata {
            title: Some(title),
            description: Some(description), 
            media: None,
            media_hash: None,
            issued_at: Some(String::from("123456")),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    #[test]
    fn test_transfer() {
        let mut context = VMContextBuilder::new();
        testing_env!(context
            .signer_account_id(accounts(0))            
            .predecessor_account_id(accounts(0))
            .build());

        let mut contract = Contract::new_default_meta(accounts(0));
        let token_md = create_token_md("ABC".into(), "Alphabet token".into());

        let token = contract.mt_mint(accounts(0),  token_md.clone(), 1000);
        assert_eq!(contract.balance_of(accounts(0), vec![token.token_id.clone()]), vec![1000], "Wrong balance");
        
        contract.register(token.token_id.clone(), accounts(1));
        assert_eq!(contract.balance_of(accounts(1), vec![token.token_id.clone()]), vec![0], "Wrong balance");

        testing_env!(context.attached_deposit(1).build());
        contract.mt_transfer(accounts(1), token.token_id.clone(), 4, None);
        
        assert_eq!(contract.balance_of(accounts(0), vec![token.token_id.clone()]), vec![996], "Wrong balance");
        assert_eq!(contract.balance_of(accounts(1), vec![token.token_id.clone()]), vec![4], "Wrong balance");
    }

    #[test]
    fn test_batch_transfer() {
        let mut context = VMContextBuilder::new();
        testing_env!(context
            .signer_account_id(accounts(0))            
            .predecessor_account_id(accounts(0))
            .build());
        let mut contract = Contract::new_default_meta(accounts(0));

        let quote_token_md = create_token_md("PYC".into(), "Python token".into());
        let base_token_md = create_token_md("ABC".into(), "Alphabet token".into());

        let quote_token = contract.mt_mint(accounts(0),  quote_token_md.clone(), 1000);
        let base_token = contract.mt_mint(accounts(0),  base_token_md.clone(), 2000);
        contract.register(quote_token.token_id.clone(), accounts(1));
        contract.register(base_token.token_id.clone(), accounts(1));
        
        testing_env!(context.attached_deposit(1).build());

        // Perform the transfers
        contract.mt_batch_transfer(
            accounts(1), 
            vec![quote_token.token_id.clone(), base_token.token_id.clone()],
            vec![4, 600],
            None,
            None
        );
    
        assert_eq!(contract.balance_of(accounts(0), vec![quote_token.token_id.clone()]), vec![996], "Wrong balance");
        assert_eq!(contract.balance_of(accounts(1), vec![quote_token.token_id.clone()]), vec![4], "Wrong balance");

        assert_eq!(contract.balance_of(accounts(0), vec![base_token.token_id.clone()]), vec![1400], "Wrong balance");
        assert_eq!(contract.balance_of(accounts(1), vec![base_token.token_id.clone()]), vec![600], "Wrong balance");
    }

    #[test]
    fn test_transfer_call() {
        // How to test a multi-contract call?
        let mut context = VMContextBuilder::new();
        testing_env!(context
            .signer_account_id(accounts(0))            
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());
        let mut contract = Contract::new_default_meta(accounts(0));
        let quote_token_md = create_token_md("ABC".into(), "Alphabet token".into());

        // alice starts with 1000, bob with 0.
        let quote_token = contract.mt_mint(accounts(0),  quote_token_md.clone(), 1000);
        contract.register(quote_token.token_id.clone(), accounts(1));

        let _result = contract.mt_transfer_call(
            accounts(1), // receiver account
            quote_token.token_id.clone(),
            100, // amount
            None,
            String::from("invest"),
        );

        
        // println!("result: {}", result);
    }

    #[test]
    fn test_batch_transfer_call() {

    }
}
