use substreams::log;
use anchor_lang::AnchorDeserialize;

use crate::{events::vaults::VaultInitEvent,
            pb::{sol::instructions::v1::Instructions,
            vault::events::v1::{Vault, Vaults}}
        };

#[substreams::handlers::map]
fn map_vault_events_from_instructions(insts: Instructions) -> Result<Vaults, substreams::errors::Error> {
    log::info!("Parsing vault event data");

    let mut vault_data = Vec::new();

    for inst in insts.instructions.iter() {
        
        let mut slice: &[u8] = &inst.event_log[..];
        let disc: [u8; 8] = {
            let mut disc = [0; 8];
            disc.copy_from_slice(&inst.event_log[..8]);
            slice = &slice[8..];
            disc
        };

        let deserialize_result: Result<VaultInitEvent, std::io::Error> = AnchorDeserialize::deserialize(&mut slice);

        match deserialize_result {
            Ok(event) => {
                vault_data.push(Vault{
                    event_id : event.event_id.to_vec(),
                    underlying_mint: event.underlying_mint.to_vec(),
                    underlying_token_acc: event.underlying_token_acc.to_vec(),
                    underlying_decimals: u32::from(event.underlying_decimals),
                    total_debt: 0,
                    total_shares: 0,
                    minimum_total_idle: 0,
                    total_idle: 0,
                    deposit_limit: event.deposit_limit,
                    min_user_deposit: event.min_user_deposit,
                    is_shutdown: false
                }); 
            },
            Err(e) => {
                log::info!("Failed to decode data: {}", e);
            }
            
        }                            
    }

    Ok(Vaults { vaults: vault_data })
}
