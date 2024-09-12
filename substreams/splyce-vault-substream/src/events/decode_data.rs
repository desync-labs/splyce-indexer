use anchor_lang::AnchorDeserialize;
use crate::{event_logs_structs::{stratagy_logs::{StrategyDepositLog, StrategyInitLog, StrategyWithdrawLog}, vault_logs::{VaultAddStrategyLog, VaultDepositLog, VaultInitLog, VaultUpdateDepositLimitLog, VaultWithdrawlLog}}, pb::vault::events::v1::{StrategyDepositEvent, StrategyInitEvent, StrategyWithdrawEvent, VaultAddStrtegyEvent, VaultDepositEvent, VaultInitEvent, VaultUpdateDepositLimitEvent, VaultWithdrawlEvent}};

use std::error::Error;

pub trait DecodeVaultData: Sized {
    fn parse_from_data(data: &mut &[u8]) -> std::result::Result<Self, Box<dyn Error>>;
    fn descriptor() -> [u8; 8];
}

//TODO: Explore option how to remove this hardcoded descriptor

impl DecodeVaultData for VaultInitEvent {

    fn descriptor() -> [u8; 8] {
         [173, 160, 208, 103, 85, 78, 229, 205]
    }

    fn parse_from_data(data: &mut &[u8]) -> std::result::Result<Self, Box<dyn Error>> {
        
        let event: VaultInitLog = AnchorDeserialize::deserialize(data)
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;    
    
        let init_event: VaultInitEvent = VaultInitEvent{
            vault_index: event.vault_index.to_vec(),
            underlying_mint: event.underlying_mint.to_vec(),
            underlying_token_acc: event.underlying_token_acc.to_vec(),
            underlying_decimals: u32::from(event.underlying_decimals),
            deposit_limit: event.deposit_limit,
            min_user_deposit: event.min_user_deposit,
        };
    
        Ok(init_event)
    }

}

impl DecodeVaultData for VaultAddStrtegyEvent {

    fn descriptor() -> [u8; 8] {
        [246, 91, 229, 44, 239, 26, 28, 150]
    }

    fn parse_from_data(data: &mut &[u8]) -> std::result::Result<Self, Box<dyn Error>> {
        
        let event: VaultAddStrategyLog = AnchorDeserialize::deserialize(data)
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;    
    
        let strategy_add_event: VaultAddStrtegyEvent = VaultAddStrtegyEvent{
            vault_index: event.vault_index.to_vec(),
            strategy_key: event.strategy_key.to_vec(),
            current_debt: event.current_debt,
            max_debt: event.max_debt,
            last_update: event.last_update,
            is_active: event.is_active,
        };
    
        Ok(strategy_add_event)
    }

}

impl DecodeVaultData for VaultDepositEvent {

    fn descriptor() -> [u8; 8] {
        [187, 186, 196, 189, 175, 44, 10, 64]
    }

    fn parse_from_data(data: &mut &[u8]) -> std::result::Result<Self, Box<dyn Error>> {
        
        let event: VaultDepositLog = AnchorDeserialize::deserialize(data)
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;    
    
        let deposit_event: VaultDepositEvent = VaultDepositEvent{
            vault_index: event.vault_index.to_vec(),
            amount: event.amount,
            share: event.share,
        };
    
        Ok(deposit_event)
    }

}

impl DecodeVaultData for VaultWithdrawlEvent {

    fn descriptor() -> [u8; 8] {
        [13, 122, 111, 4, 123, 191, 191, 248]
    }

    fn parse_from_data(data: &mut &[u8]) -> std::result::Result<Self, Box<dyn Error>> {
        
        let event: VaultWithdrawlLog = AnchorDeserialize::deserialize(data)
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;    
    
        let withdraw_event: VaultWithdrawlEvent = VaultWithdrawlEvent{
            vault_index: event.vault_index.to_vec(),
            total_idle: event.total_idle,
            total_share: event.total_share,
            assets_to_transfer: event.assets_to_transfer,
            shares_to_burn: event.shares_to_burn,
        };
    
        Ok(withdraw_event)
    }

}

impl DecodeVaultData for VaultUpdateDepositLimitEvent {

    fn descriptor() -> [u8; 8] {
        [19, 98, 248, 35, 149, 145, 56, 26]
    }

    fn parse_from_data(data: &mut &[u8]) -> std::result::Result<Self, Box<dyn Error>> {
        
        let event: VaultUpdateDepositLimitLog = AnchorDeserialize::deserialize(data)
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;    
    
        let update_limit_event: VaultUpdateDepositLimitEvent = VaultUpdateDepositLimitEvent{
            vault_index: event.vault_index.to_vec(),
            new_limit: event.new_limit,
        };
    
        Ok(update_limit_event)
    }

}


impl DecodeVaultData for StrategyInitEvent {

    fn descriptor() -> [u8; 8] {
        [33, 61, 4, 77, 20, 107, 154, 62]
    }

    fn parse_from_data(data: &mut &[u8]) -> std::result::Result<Self, Box<dyn Error>> {
        
        let event: StrategyInitLog = AnchorDeserialize::deserialize(data)
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;    
    
        let init_event: StrategyInitEvent = StrategyInitEvent{
            account_key: event.account_key.to_vec(),
            strategy_type: event.strategy_type,
            vault : event.vault.to_vec(),
            underlying_mint: event.underlying_mint.to_vec(),
            underlying_token_acc: event.underlying_token_acc.to_vec(),
            underlying_decimals: u32::from(event.undelying_decimals),
            deposit_limit: event.deposit_limit,
            deposit_period_ends: event.deposit_period_ends,
            lock_period_ends: event.lock_period_ends,
        };
    
        Ok(init_event)
    }

}

impl DecodeVaultData for StrategyDepositEvent {

    fn descriptor() -> [u8; 8] {
        [44, 150, 97, 77, 190, 106, 76, 237]
    }

    fn parse_from_data(data: &mut &[u8]) -> std::result::Result<Self, Box<dyn Error>> {
        
        let event: StrategyDepositLog = AnchorDeserialize::deserialize(data)
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;    
    
        let deposit_event: StrategyDepositEvent = StrategyDepositEvent { 
            account_key: event.account_key.to_vec(), 
            amount: event.amount, 
            total_assets: event.total_assets 
        };
    
        Ok(deposit_event)
    }

}

impl DecodeVaultData for StrategyWithdrawEvent {

    fn descriptor() -> [u8; 8] {
        [120, 188, 132, 45, 215, 160, 115, 81]
    }

    fn parse_from_data(data: &mut &[u8]) -> std::result::Result<Self, Box<dyn Error>> {
        
        let event: StrategyWithdrawLog = AnchorDeserialize::deserialize(data)
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;    
    
        let withdraw_event: StrategyWithdrawEvent = StrategyWithdrawEvent { 
            account_key: event.account_key.to_vec(), 
            amount: event.amount, 
            total_assets: event.total_assets 
        };
    
        Ok(withdraw_event)
    }
}
