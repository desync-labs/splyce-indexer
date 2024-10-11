use anchor_lang::prelude::*;


#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct VaultInitLog {
    pub vault_index: [u8; 8],
    pub underlying_mint: [u8 ; 32],
    pub underlying_token_acc: [u8 ; 32],
    pub underlying_decimals: u8,
    pub share_mint: [u8 ; 32],
    pub share_token_acc: [u8 ; 32],
    pub share_decimals: u8,
    pub deposit_limit: u64,
    pub min_user_deposit: u64,
    pub performance_fee: u64,
    pub vault_pda: [u8 ; 32],
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
pub struct UpdatedCurrentDebtForStrategyLog {
  pub vault_index: [u8; 8],
  pub strategy_key: Pubkey,
  pub total_idle: u64,
  pub total_debt: u64,
  pub new_debt: u64,
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct VaultDepositLog {
    pub vault_index: [u8; 8],
    pub total_debt: u64,
    pub total_idle: u64,
    pub total_share: u64,
    pub amount: u64,
    pub share: u64,
    pub token_account: [u8 ; 32],
    pub share_account: [u8 ; 32],
    pub authority: [u8 ; 32],
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct VaultWithdrawlLog {
    pub vault_index: [u8; 8],
    pub total_idle: u64,
    pub total_share: u64,
    pub assets_to_transfer: u64,
    pub shares_to_burn: u64,
    pub token_account: [u8 ; 32],
    pub share_account: [u8 ; 32],
    pub authority: [u8 ; 32],
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct VaultUpdateDepositLimitLog {
    pub vault_index: [u8; 8],
    pub new_limit: u64,
}