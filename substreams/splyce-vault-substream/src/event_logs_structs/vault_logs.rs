use anchor_lang::prelude::*;


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
    pub last_update: i64,
    pub is_active: bool,    
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct VaultDepositLog {
    pub vault_index: [u8; 8],
    pub amount: u64,
    pub share: u64,
    pub depositor: [u8 ; 32],
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct VaultWithdrawlLog {
    pub vault_index: [u8; 8],
    pub total_idle: u64,
    pub total_share: u64,
    pub assets_to_transfer: u64,
    pub shares_to_burn: u64,
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct VaultUpdateDepositLimitLog {
    pub vault_index: [u8; 8],
    pub new_limit: u64,
}