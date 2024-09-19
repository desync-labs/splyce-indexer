use anchor_lang::AnchorDeserialize;
use crate::{event_logs_structs::{stratagy_logs::{StrategyDepositLog, StrategyInitLog, StrategyWithdrawLog}, vault_logs::{VaultAddStrategyLog, VaultDepositLog, VaultInitLog, VaultUpdateDepositLimitLog, VaultWithdrawlLog}}, pb::vault::events::v1::{StrategyDepositEvent, StrategyInitEvent, StrategyWithdrawEvent, VaultAddStrategyEvent, VaultDepositEvent, VaultInitEvent, VaultUpdateDepositLimitEvent, VaultWithdrawlEvent}};

use std::error::Error;
use substreams::log;
use crate::utils::utils;

pub trait DecodeVaultData: Sized {
    fn parse_from_data(data: &mut &[u8]) -> std::result::Result<Self, Box<dyn Error>>;
    fn descriminator() -> [u8; 8];
}

//TODO: Check how to make this file modular and avoid the need to implement the same function for each event
impl DecodeVaultData for VaultInitEvent {

    fn descriminator() -> [u8; 8] {
        utils::get_descriminator("VaultInitEvent")
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

impl DecodeVaultData for VaultAddStrategyEvent {

    fn descriminator() -> [u8; 8] {
        utils::get_descriminator("VaultAddStrategyEvent")
    }

    fn parse_from_data(data: &mut &[u8]) -> std::result::Result<Self, Box<dyn Error>> {
        
        let event: VaultAddStrategyLog = AnchorDeserialize::deserialize(data)
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;    
    
        let strategy_add_event: VaultAddStrategyEvent = VaultAddStrategyEvent{
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

    fn descriminator() -> [u8; 8] {
        utils::get_descriminator("VaultDepositEvent")
    }

    fn parse_from_data(data: &mut &[u8]) -> std::result::Result<Self, Box<dyn Error>> {
        
        let event: VaultDepositLog = AnchorDeserialize::deserialize(data)
                        .map_err(|e| Box::new(e) as Box<dyn Error>)?;    
    
        let deposit_event: VaultDepositEvent = VaultDepositEvent{
            vault_index: event.vault_index.to_vec(),
            amount: event.amount,
            share: event.share,
            depositor: event.depositor.to_vec(),
        };
    
        Ok(deposit_event)
    }

}

impl DecodeVaultData for VaultWithdrawlEvent {

    fn descriminator() -> [u8; 8] {
        utils::get_descriminator("VaultWithdrawlEvent")
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

    fn descriminator() -> [u8; 8] {
        utils::get_descriminator("VaultUpdateDepositLimitEvent")
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

    fn descriminator() -> [u8; 8] {
        utils::get_descriminator("StrategyInitEvent")
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

    fn descriminator() -> [u8; 8] {
        utils::get_descriminator("StrategyDepositEvent")
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

    fn descriminator() -> [u8; 8] {
        utils::get_descriminator("StrategyWithdrawEvent")
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
