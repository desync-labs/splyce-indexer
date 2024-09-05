use std::vec;

use crate::pb::{sol::{instructions::v1::{Instruction, Instructions}, transactions::v1::Transactions}, vault::events::v1::VaultEventLogs};
use anchor_lang::AnchorSerialize;
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
