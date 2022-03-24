/*! Multi-Token Implementation (ERC-1155)

*/

mod core_impl;
mod receiver;
mod resolver;

pub use self::core_impl::*;

pub use self::receiver::*;
pub use self::resolver::*;

use crate::multi_token::token::TokenId;
use near_sdk::{AccountId, Balance, PromiseOrValue};
use near_sdk::json_types::U128;

use super::token::Token;

/// Describes functionality according to this - https://eips.ethereum.org/EIPS/eip-1155
/// And this - <https://github.com/shipsgold/NEPs/blob/master/specs/Standards/MultiToken/Core.md>
pub trait MultiTokenCore {

    /// Make a single transfer
    ///
    /// # Arguments
    ///
    /// * `receiver_id`: the valid NEAR account receiving the token
    /// * `token_id`: ID of the token to transfer
    /// * `amount`: the number of tokens to transfer
    ///
    /// returns: ()
    ///
    fn mt_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        amount: Balance,
        approval: Option<u64>,
        memo: Option<String>
    );

    // Make a batch transfer
    // Behaves similar 
    fn mt_batch_transfer(
        &mut self,
        receiver_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        approval_ids: Option<Vec<Option<u64>>>,
        memo: Option<String>,
        msg: String
    );

    /// Transfer MT and call a method on receiver contract. A successful
    /// workflow will end in a success execution outcome to the callback on the MT
    /// contract at the method `resolve_transfer`.
    ///
    /// # Arguments
    ///
    /// * `receiver_id`: NEAR account receiving MT
    /// * `token_id`: Token to send
    /// * `amount`: How much to send
    /// * `approval_id`: ID of approval for signer
    /// * `memo`: Used as context
    /// * `msg`: Additional msg that will be passed to receiving contract
    ///
    /// returns: PromiseOrValue<bool>
    ///
    fn mt_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        amount: Balance,
        approval_id: Option<u64>,
        msg: String
    ) -> PromiseOrValue<bool>;

    fn mt_batch_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        memo: Option<String>,
        msg: String
    ) -> PromiseOrValue<Vec<u128>>;


    // View Methods
    fn mt_token(&self, token_ids: Vec<TokenId>) -> Vec<Option<Token>>;

    fn mt_balance_of(&self, account_id: AccountId, token_id: TokenId) -> U128;

    fn mt_batch_balance_of(&self, account_id: AccountId, token_ids: Vec<TokenId>) -> Vec<U128>;

    fn mt_supply(&self, token_id: TokenId) -> Option<U128>;
    
    fn mt_batch_supply(&self, token_ids: Vec<TokenId>) -> Vec<Option<U128>>;
}
