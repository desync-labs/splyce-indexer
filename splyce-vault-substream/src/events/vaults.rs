use anchor_lang::prelude::*;

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct VaultInitLog {
    pub event_id: [u8 ; 8],    
    pub underlying_mint: [u8 ; 32],
    pub underlying_token_acc: [u8 ; 32],
    pub underlying_decimals: u8,
    pub deposit_limit: u64,
    pub min_user_deposit: u64,
}