use crate::pb::sol::instructions::v1::{Instruction, Instructions};
use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use crate::utils::utils::read_descriptor;


#[substreams::handlers::map]
fn filtered_instructions_without_votes(
    query: String,
    block: Block,
) -> Result<Instructions, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    // Create a vector to store filtered instructions
    let mut filtered_instructions = Vec::new();

    // Filter and collect instructions
    for trx in block.transactions {
        //======================================================================
        //          Extract the log messages, filter, and process them
        //======================================================================
        let program_event_logs = trx.meta.as_ref()
        .map_or_else(
            || vec![], // Return an empty vector if `meta` is `None`
            |meta| meta.log_messages
                .iter()
                .filter_map(|log| {
                    // Check if the log starts with the prefix
                    if log.starts_with("Program data:") {
                        // Remove the prefix and trim whitespace
                        let remaining_log = log["Program data:".len()..].trim().to_string();
                        Some(remaining_log)
                    } else {
                        None
                    }
                })
                .collect() // Collect results into a vector
        );

        //log::info!("Program Event Logs = {:?}", program_event_logs);

        //======================================================================
        //          Collect instructions and filter based on query
        //======================================================================
        let instructions: Vec<Instruction> = trx.walk_instructions()
            .filter_map(|view| {
                let program_id = view.program_id().to_string();
                let accounts = view.accounts().iter().map(|acct| acct.to_string()).collect();
                let data = view.data().to_vec();
                //let tx_hash = trx.hash().to_string();

                if query.matches_keys(&vec![format!("program:{}", program_id.clone())]) {

                    //======================================================================
                    //          Collect instructions and filter based on query
                    //======================================================================
                    let mut event_log: Vec<u8> = vec![];
                    program_event_logs.iter().for_each(|log: &String| {
                        // log::info!("Cleanup program event logs {:?}", log);
                        let borsh_bytes = base64::decode(log).unwrap();
                        // log::info!("borsh_bytes event logs {:?}", borsh_bytes);

                        // Define the starting index and length of the slice
                        let start_index = 8; // Starting index for the 9th element (0-based index)
                        let length = 8; // Length of the slice

                        // Obtain the slice
                        let sub_slice: &[u8] = &borsh_bytes[start_index..start_index + length];

                        let discriminator = read_descriptor(&view.data());

                        if sub_slice == discriminator {
                            log::info!("Discriminator matched");
                            event_log = borsh_bytes.clone();
                        } else {
                            log::info!("Discriminator did not match");
                        }
                    });
                    
                    log::info!("Discriminator = {:?}", read_descriptor(&view.data()));

                    Some(Instruction {
                        program_id,
                        accounts,
                        data,
                        tx_hash: String::from("0x00"), //TODO: Add tx_hash
                        event_log,
                    })
                } else {
                    None
                }
            })
            .collect();

        // Add the filtered instructions to the final vector
        filtered_instructions.extend(instructions);
    }

    Ok(Instructions { instructions: filtered_instructions })
}
