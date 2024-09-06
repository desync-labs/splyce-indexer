use anchor_lang::AnchorDeserialize;
use crate::pb::vault::events::v1::{VaultAddStrtegyEvent, VaultDepositEvent, VaultInitEvent, VaultWithdrawlEvent};
use std::error::Error;
use super::vaults::{VaultAddStrategyLog, VaultDepositLog, VaultInitLog, VaultWithdrawlLog};
use substreams::log;

pub trait DecodeVaultData: Sized {
    fn parse_from_data(data: &mut &[u8]) -> std::result::Result<Self, Box<dyn Error>>;
    fn descriptor() -> [u8; 8];
}

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