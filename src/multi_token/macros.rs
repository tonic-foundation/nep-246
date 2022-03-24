/// The core methods for a basic multi token. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_multi_token_core {
    ($contract: ident, $token: ident) => {
        use $crate::multi_token::core::MultiTokenCore;
        use $crate::multi_token::core::MultiTokenResolver;

        #[near_bindgen]
        impl MultiTokenCore for $contract {
            #[payable]
            fn mt_transfer(
                &mut self,
                receiver_id: AccountId,
                token_id: TokenId,
                amount: U128,
                approval: Option<u64>,
                memo: Option<String>
            ) {
                self.$token.mt_transfer(receiver_id, token_id, amount, approval, memo)
            }

            #[payable]
            fn mt_batch_transfer(
                &mut self,
                receiver_id: AccountId,
                token_ids: Vec<TokenId>,
                amounts: Vec<U128>,
                approval_ids: Option<Vec<Option<u64>>>,
                memo: Option<String>,
            ) {
                self.$token.mt_batch_transfer(receiver_id, token_ids, amounts, approval_ids, memo)
            }

            #[payable]
            fn mt_transfer_call(
                &mut self,
                receiver_id: AccountId,
                token_id: TokenId,
                amount: Balance,
                approval_id: Option<u64>,
                msg: String
            ) -> PromiseOrValue<bool> {
                self.$token.mt_transfer_call(receiver_id, token_id, amount, approval_id, msg)
            }

            #[payable]
            fn mt_batch_transfer_call(
                &mut self,
                receiver_id: AccountId,
                token_ids: Vec<TokenId>,
                amounts: Vec<U128>,
                memo: Option<String>,
                msg: String
            ) -> PromiseOrValue<Vec<u128>> {
                self.$token.mt_batch_transfer_call(receiver_id, token_ids, amounts, memo, msg)
            }

            fn mt_token(&self, token_ids: Vec<TokenId>) -> Vec<Option<Token>> {
                self.$token.mt_token(token_ids)
            }

            fn mt_balance_of(&self, account_id: AccountId, token_id: TokenId) -> U128 {
                self.$token.mt_balance_of(account_id, token_id)
            }

            fn mt_batch_balance_of(&self, account_id: AccountId, token_ids: Vec<TokenId>) -> Vec<U128> {
                self.$token.mt_batch_balance_of(account_id, token_ids)
            }

            fn mt_supply(&self, token_id: TokenId) -> Option<U128> {
                self.$token.mt_supply(token_id)
            }

            fn mt_batch_supply(&self, token_ids: Vec<TokenId>) -> Vec<Option<U128>> {
                self.$token.mt_batch_supply(token_ids)
            }
        }

        #[near_bindgen]
        impl MultiTokenResolver for $contract {
            #[private]
            fn mt_resolve_transfer(
                &mut self,
                sender_id: AccountId,
                receiver_id: AccountId,
                token_ids: Vec<TokenId>,
                amounts: Vec<U128>,
                approvals: Option<Vec<Option<Approval>>>,
            ) -> Vec<U128> {
                self.$token.mt_resolve_transfer(
                    sender_id,
                    receiver_id,
                    token_ids,
                    amounts,
                    approvals
                )
            }
        }
    };
}

/// Multi token approval management allows for an escrow system where
/// multiple approvals per token exist.
#[macro_export]
macro_rules! impl_multi_token_approval {
    ($contract: ident, $token: ident) => {
        use $crate::multi_token::approval::MultiTokenApproval;

        #[near_bindgen]
        impl MultiTokenApproval for $contract {
            #[payable]
            fn approve(
                &mut self,
                account_id: AccountId,
                token_id: TokenId,
                amount: Balance,
                msg: Option<String>,
            ) -> Option<Promise> {
                self.$token.approve(account_id, token_id, amount, msg)
            }

            #[payable]
            fn revoke(&mut self, token_id: TokenId, account_id: AccountId) {
                self.$token.revoke(token_id, account_id)
            }

            #[payable]
            fn revoke_all(&mut self, token_id: TokenId) {
                self.$token.revoke_all(token_id)
            }

            fn is_approved(
                &self,
                token_id: TokenId,
                approved_account_id: AccountId,
                amount: Balance,
                approval: Option<u64>,
            ) -> bool {
                self.$token.is_approved(token_id, approved_account_id, amount, approval)
            }
        }
    };
}

/// Multi-token enumeration adds the extension standard offering several
/// view-only methods to get token supply, tokens per owner, etc.
#[macro_export]
macro_rules! impl_multi_token_enumeration {
    ($contract: ident, $token: ident) => {
        use $crate::multi_token::enumeration::MultiTokenEnumeration;

        #[near_bindgen]
        impl MultiTokenEnumeration for $contract {
            fn tokens(&self, from_index: Option<u64>, limit: u64) -> Vec<Token> {
                self.$token.tokens(from_index, limit)
            }

            fn token_by_owner(&self, account_id: AccountId, from_index: Option<u64>, limit: u64) -> Vec<Token> {
                self.$token.token_by_owner(account_id, from_index, limit)
            }
        }
    };
}