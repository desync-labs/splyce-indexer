use anchor_lang::prelude::*;


#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct StrategyInitLog {
    pub account_key: [u8 ; 32],
    pub strategy_type: String,
    pub vault: [u8 ; 32],
    pub underlying_mint: [u8 ; 32],
    pub underlying_token_acc: [u8 ; 32],
    pub undelying_decimals: u8,
    pub total_idle: u64,
    pub total_funds: u64,
    pub deposit_limit: u64,
    pub deposit_period_ends: u64,
    pub lock_period_ends: u64,
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct StrategyDepositLog {
    pub account_key: [u8 ; 32],
    pub amount: u64,
    pub total_funds: u64,
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct StrategyWithdrawLog {
    pub account_key: [u8 ; 32],
    pub amount: u64,
    pub total_funds: u64,
}