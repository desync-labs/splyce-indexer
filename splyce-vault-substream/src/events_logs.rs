use std::vec;

use crate::{events::vaults::VaultInitLog, pb::{sol::{instructions::v1::{Instruction, Instructions}, transactions::v1::Transactions}, vault::events::v1::{vault_event, VaultEvent, VaultEventLogs, VaultInitEvent}}};
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::utils::utils::read_descriptor;

#[substreams::handlers::map]
fn filtered_event_logs(
    txns: Transactions,
) -> Result<VaultEventLogs, substreams::errors::Error> {
    let mut program_event_logs1: Vec<Vec<u8>> = vec![vec![]];
    // Filter and collect event logs
    for trx in txns.transactions {
        let program_event_logs: Vec<Vec<u8>> =  trx.meta.as_ref()
            .map_or_else(
            || vec![], // Return an empty vector if `meta` is `None`
            |meta| meta.log_messages
            .iter()
            .filter_map(|log| {
                // Check if the log starts with the prefix
                if log.starts_with("Program data:") {
                    // Remove the prefix and trim whitespace
                    let remaining_log = log["Program data:".len()..].trim().to_string();
                    let borsh_bytes = base64::decode(&remaining_log).unwrap();

                    let discriminator = read_descriptor(&borsh_bytes);
                    log::info!("Event Discriminator = {:?}", discriminator);

                    Some(borsh_bytes)
                } else {
                    None
                }
            })
            .collect() // Collect results into a vector
        );

        program_event_logs1.extend(program_event_logs)
    }

    Ok(VaultEventLogs {
        logs: program_event_logs1,
    })
}

#[substreams::handlers::map]
fn map_vault_events_from_logs(logs: VaultEventLogs) -> Result<VaultEvent, substreams::errors::Error> {
    log::info!("Parsing vault event data");

    let mut vault_event:VaultEvent = VaultEvent::default();

    for log in logs.logs.iter() {
        
        if log.len() < 8 {
            log::info!("Invalid log data");
            continue;
            
        }

        let mut slice: &[u8] = &log[..];
        let disc: [u8; 8] = {
            let mut disc = [0; 8];
            disc.copy_from_slice(&log[..8]);
            slice = &slice[8..];
            disc
        };

        //TODO: put a switch case or some generic approach here.
        let deserialize_log: Result<VaultInitLog, std::io::Error> = AnchorDeserialize::deserialize(&mut slice);
        
        match deserialize_log {
            Ok(event) => {
                let init_event = VaultInitEvent{
                    // account_id: inst.accounts[0].to_string(),
                    underlying_mint: event.underlying_mint.to_vec(),
                    underlying_token_acc: event.underlying_token_acc.to_vec(),
                    underlying_decimals: u32::from(event.underlying_decimals),
                    deposit_limit: event.deposit_limit,
                    min_user_deposit: event.min_user_deposit,
                };
                vault_event.event = Some(vault_event::Event::Initialize(init_event)); 
            },
            Err(e) => {
                log::info!("Failed to decode data: {}", e);
            }
        }  
    }

    Ok(vault_event)
}