use anchor_lang::AnchorDeserialize;
use crate::pb::vault::events::v1::VaultInitEvent;
use std::error::Error;
use super::vaults::VaultInitLog;
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
            underlying_mint: event.underlying_mint.to_vec(),
            underlying_token_acc: event.underlying_token_acc.to_vec(),
            underlying_decimals: u32::from(event.underlying_decimals),
            deposit_limit: event.deposit_limit,
            min_user_deposit: event.min_user_deposit,
        };
    
        Ok(init_event)
    }

}