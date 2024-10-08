use anchor_lang::prelude::*;


#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct StrategyInitLog {
    pub account_key: [u8 ; 32],
    pub strategy_type: String,
    pub vault: [u8 ; 32],
    pub underlying_mint: [u8 ; 32],
    pub underlying_token_acc: [u8 ; 32],
    pub undelying_decimals: u8,
    pub deposit_limit: u64,
    pub deposit_period_ends: i64,
    pub lock_period_ends: i64,
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct StrategyDepositLog {
    pub account_key: [u8 ; 32],
    pub amount: u64,
    pub total_assets: u64,
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct StrategyWithdrawLog {
    pub account_key: [u8 ; 32],
    pub amount: u64,
    pub total_assets: u64,
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct StrategyReportedLog {
    pub strategy_key: [u8 ; 32],
    pub gain: u64,
    pub loss: u64,
    pub current_debt: u64,
    pub protocol_fees: u64,
    pub total_fees: u64,
    pub timestamp: i64,
}