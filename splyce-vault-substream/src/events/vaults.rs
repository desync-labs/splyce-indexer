use std::vec;
use std::error::Error;
use anchor_lang::prelude::*;
use crate::pb::vault::events::v1::VaultInitEvent;



#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct VaultInitLog {
    pub underlying_mint: [u8 ; 32],
    pub underlying_token_acc: [u8 ; 32],
    pub underlying_decimals: u8,
    pub deposit_limit: u64,
    pub min_user_deposit: u64,
}