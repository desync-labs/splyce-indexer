use std::vec;
use std::error::Error;
use anchor_lang::prelude::*;
use crate::pb::vault::events::v1::VaultInitEvent;



#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct VaultInitLog {
    pub vault_index: [u8; 8],
    pub underlying_mint: [u8 ; 32],
    pub underlying_token_acc: [u8 ; 32],
    pub underlying_decimals: u8,
    pub deposit_limit: u64,
    pub min_user_deposit: u64,
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct VaultAddStrategyLog {
    pub vault_index: [u8; 8],
    pub strategy_key: [u8 ; 32],
    pub current_debt: u64,
    pub max_debt: u64,
    pub last_update: u64,
    pub is_active: bool,    
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct VaultDepositLog {
    pub vault_index: [u8; 8],
    pub amount: u64,
    pub share: u64,
}