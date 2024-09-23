use std::vec;
use substreams::log;
use std::error::Error;

use crate::{events::decode_data::DecodeVaultData, pb::{sol::transactions::v1::Transactions, vault::events::v1::{vault_event, StrategyDepositEvent, StrategyInitEvent, StrategyWithdrawEvent, VaultAddStrategyEvent, VaultDepositEvent, VaultEvent, VaultEventLogs, VaultInitEvent, VaultUpdateDepositLimitEvent, VaultWithdrawlEvent, UpdatedCurrentDebtForStrategyEvent}}, utils::utils::read_descriminator};

#[substreams::handlers::map]
fn filtered_event_logs(
    txns: Transactions,
) -> Result<VaultEventLogs, substreams::errors::Error> {

    let mut program_event_logs1: Vec<Vec<u8>> = vec![];
    let mut trx_hash = "0x".to_string();
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

                    if let Some(transaction) = &trx.transaction { 
                        trx_hash = bs58::encode(&transaction.signatures[0]).into_string();
                    }
                    
                    Some(borsh_bytes)
                } else {
                    None
                }
            })
            .collect() // Collect results into a vector
        );

        program_event_logs1.extend(program_event_logs);   

    }

    Ok(VaultEventLogs {
        logs: program_event_logs1,
        transaction_hash:trx_hash,
        block_height: txns.block_height,
        block_timestamp: txns.block_timestamp,
    })
    
}

#[substreams::handlers::map]
fn map_vault_events_from_logs(logs: VaultEventLogs) -> Result<VaultEvent, substreams::errors::Error> {
    let mut vault_event:VaultEvent = VaultEvent::default();

    for log in logs.logs.iter() {
        
        if log.len() < 8 {
            log::debug!("Invalid log data");
            continue;
        }

        vault_event = decode_and_parse(log);
        vault_event.transaction_hash = logs.transaction_hash.clone();
        vault_event.block_height = logs.block_height.clone();
        vault_event.block_timestamp = logs.block_timestamp.clone();
    }

    Ok(vault_event)
}

//TODO: Move this to a separate file
fn decode_and_parse(log: &Vec<u8>) -> VaultEvent{
    let mut vault_event:VaultEvent = VaultEvent::default();
    
    let mut slice: &[u8] = &log[..];
    let disc: [u8; 8] = {
        let mut disc = [0; 8];
        disc.copy_from_slice(&log[..8]);
        slice = &slice[8..];
        disc
    };

   if VaultInitEvent::descriminator() == disc{
        match decode_and_parse_to_protobuf::<VaultInitEvent>(&mut slice) {
            Ok(parsed_event) => {
                vault_event.event = Some(vault_event::Event::VaultInitialize(parsed_event))
            },
            Err(e) => {
                log::info!("Failed to decode vault init data: {}", e);
            }
        }
    }else if VaultAddStrategyEvent::descriminator() == disc{
        match decode_and_parse_to_protobuf::<VaultAddStrategyEvent>(&mut slice) {
            Ok(parsed_event) => {
                vault_event.event = Some(vault_event::Event::StrategyAdd(parsed_event))
            },
            Err(e) => {
                log::info!("Failed to decode vault strategy add data: {}", e);
            }
        }
    }else if VaultDepositEvent::descriminator() == disc{
        match decode_and_parse_to_protobuf::<VaultDepositEvent>(&mut slice) {
            Ok(parsed_event) => {
                vault_event.event = Some(vault_event::Event::VaultDeposit(parsed_event))
            },
            Err(e) => {
                log::info!("Failed to decode vault deposit event data: {}", e);
            }
        }
    }else if VaultWithdrawlEvent::descriminator() == disc{
        match decode_and_parse_to_protobuf::<VaultWithdrawlEvent>(&mut slice) {
            Ok(parsed_event) => {
                vault_event.event = Some(vault_event::Event::Withdrwal(parsed_event))
            },
            Err(e) => {
                log::info!("Failed to decode vault withdrawl event data: {}", e);
            }
        }
    }else if VaultUpdateDepositLimitEvent::descriminator() == disc{
        match decode_and_parse_to_protobuf::<VaultUpdateDepositLimitEvent>(&mut slice) {
            Ok(parsed_event) => {
                vault_event.event = Some(vault_event::Event::UpdateDepositLimit(parsed_event))
            },
            Err(e) => {
                log::info!("Failed to decode vault update limit event data: {}", e);
            }
        }
    }else if StrategyInitEvent::descriminator() == disc{
        match decode_and_parse_to_protobuf::<StrategyInitEvent>(&mut slice) {
            Ok(parsed_event) => {
                vault_event.event = Some(vault_event::Event::StrategyInitialize(parsed_event))
            },
            Err(e) => {
                log::info!("Failed to decode vault strategy init event data: {}", e);
            }
        }
    }else if StrategyDepositEvent::descriminator() == disc{
        match decode_and_parse_to_protobuf::<StrategyDepositEvent>(&mut slice) {
            Ok(parsed_event) => {
                vault_event.event = Some(vault_event::Event::StrategyDeposit(parsed_event))
            },
            Err(e) => {
                log::info!("Failed to decode vault strategy deposit event data: {}", e);
            }
        }
    }else if StrategyWithdrawEvent::descriminator() == disc{
        match decode_and_parse_to_protobuf::<StrategyWithdrawEvent>(&mut slice) {
            Ok(parsed_event) => {
                vault_event.event = Some(vault_event::Event::StrategyWithdraw(parsed_event))
            },
            Err(e) => {
                log::info!("Failed to decode vault strategy withdraw event data: {}", e);
            }
        }
    }else if UpdatedCurrentDebtForStrategyEvent::descriminator() == disc{
        match decode_and_parse_to_protobuf::<UpdatedCurrentDebtForStrategyEvent>(&mut slice) {
            Ok(parsed_event) => {
                vault_event.event = Some(vault_event::Event::UpdatedDebtForStrategy(parsed_event))
            },
            Err(e) => {
                log::info!("Failed to decode strategy debt update event data: {}", e);
            }
        }
    }

    vault_event
}

fn decode_and_parse_to_protobuf<T: DecodeVaultData>(data: &mut &[u8]) -> Result<T, Box<dyn Error>> {
    T::parse_from_data(data)
}

