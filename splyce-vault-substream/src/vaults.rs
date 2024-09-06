use substreams::log;
use anchor_lang::AnchorDeserialize;
use crate::{events::vaults::VaultInitLog,
            pb::{sol::instructions::v1::Instructions, vault::events::v1::{vault_event, VaultEvent, VaultInitEvent},
        }
        };

#[substreams::handlers::map]
fn map_vault_events_from_instructions(insts: Instructions) -> Result<VaultEvent, substreams::errors::Error> {
    log::info!("Parsing vault event data");

    let mut vault_event:VaultEvent = VaultEvent::default();

    for inst in insts.instructions.iter() {
        
        let mut slice: &[u8] = &inst.event_log[..];
        let disc: [u8; 8] = {
            let mut disc = [0; 8];
            disc.copy_from_slice(&inst.event_log[..8]);
            slice = &slice[8..];
            disc
        };

        //TODO: put a switch case or some generic approach here.
        let deserialize_log: Result<VaultInitLog, std::io::Error> = AnchorDeserialize::deserialize(&mut slice);
        
        match deserialize_log {
            Ok(event) => {
                let init_event = VaultInitEvent{
                    vault_index: event.vault_index.to_vec(),
                    underlying_mint: event.underlying_mint.to_vec(),
                    underlying_token_acc: event.underlying_token_acc.to_vec(),
                    underlying_decimals: u32::from(event.underlying_decimals),
                    deposit_limit: event.deposit_limit,
                    min_user_deposit: event.min_user_deposit,
                };
                vault_event.event = Some(vault_event::Event::VaultInitialize(init_event)); 
            },
            Err(e) => {
                log::info!("Failed to decode data: {}", e);
            }
        }  
    }

    Ok(vault_event)
}
