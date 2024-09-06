use std::vec;
use substreams::log;
use std::error::Error;

use crate::{events::decode_data::DecodeVaultData, pb::{sol::transactions::v1::Transactions, vault::events::v1::{vault_event, VaultAddStrtegyEvent, VaultDepositEvent, VaultEvent, VaultEventLogs, VaultInitEvent}}};
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

        vault_event = decode_and_parse(log);
    }

    Ok(vault_event)
}

fn decode_and_parse(log: &Vec<u8>) -> VaultEvent{
    let mut vault_event:VaultEvent = VaultEvent::default();
    
    let mut slice: &[u8] = &log[..];
    let disc: [u8; 8] = {
        let mut disc = [0; 8];
        disc.copy_from_slice(&log[..8]);
        slice = &slice[8..];
        disc
    };

    log::info!("<<Instruction disriminator>> : {:?}", disc);

   if VaultInitEvent::descriptor() == disc{
        match decode_and_parse_to_protobuf::<VaultInitEvent>(&mut slice) {
            Ok(parsed_event) => {
                vault_event.event = Some(vault_event::Event::Initialize(parsed_event))
            },
            Err(e) => {
                log::info!("Failed to decode vault init data: {}", e);
            }
        }
    }else if VaultAddStrtegyEvent::descriptor() == disc{
        match decode_and_parse_to_protobuf::<VaultAddStrtegyEvent>(&mut slice) {
            Ok(parsed_event) => {
                vault_event.event = Some(vault_event::Event::StrategyAdd(parsed_event))
            },
            Err(e) => {
                log::info!("Failed to decode vault strategy add data: {}", e);
            }
        }
    }else if VaultDepositEvent::descriptor() == disc{
        match decode_and_parse_to_protobuf::<VaultDepositEvent>(&mut slice) {
            Ok(parsed_event) => {
                vault_event.event = Some(vault_event::Event::VaultDeposit(parsed_event))
            },
            Err(e) => {
                log::info!("Failed to decode vault deposit event data: {}", e);
            }
        }
    }

    vault_event
}

fn decode_and_parse_to_protobuf<T: DecodeVaultData>(data: &mut &[u8]) -> Result<T, Box<dyn Error>> {
    T::parse_from_data(data)
}