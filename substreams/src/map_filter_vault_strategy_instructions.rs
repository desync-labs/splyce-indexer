use anyhow::anyhow;
// use base64::{engine::general_purpose, Engine};
use serde::Deserialize;
use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, CompiledInstruction};
use crate::{pb::sol::transactions::v1::{Instruction, Instructions}, utils::utils::read_descriptor};

#[derive(Deserialize, Debug)]
struct InstructionFilterParams {
    program_id: Option<String>,
}

#[substreams::handlers::map]
fn map_filter_vault_strategy_instructions(params: String, blk: Block) -> Result<Instructions, substreams::errors::Error> {
    //log::info!("Filtering vault stratagy instructions");
    let filters = parse_filters_from_params(params)?;

    let instructions : Vec<Instruction> = blk.transactions().flat_map(|tx| {
        let msg = tx.transaction.as_ref().unwrap().message.as_ref().unwrap();
        let acct_keys = tx.resolved_accounts();

        msg.instructions.iter()
            .filter(|inst| apply_filter(inst, &filters, &acct_keys))
            .map(|inst| {


            // Extract the log messages, filter, and process them
            let program_event_logs = tx.meta.as_ref()
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

            //TODO: Simplyfy this code by mering with above iterator
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

                let discriminator = read_descriptor(&inst.data); 
                if sub_slice == discriminator {
                    log::info!("Discriminator matched");
                    event_log = borsh_bytes.clone();
                } else {
                    log::info!("Discriminator did not match");
                }
            });

            Instruction {
                program_id: bs58::encode(acct_keys[inst.program_id_index as usize].to_vec()).into_string(),
                accounts: inst.accounts.iter().map(|acct| bs58::encode(acct_keys[*acct as usize].to_vec()).into_string()).collect(),
                data: inst.data.clone(),//bs58::encode(&inst.data).into_string(),
                b58_encoded_data: bs58::encode(&inst.data).into_string(),
                event_log: event_log,
            }
        }).collect::<Vec<_>>()
    }).collect();

    Ok(Instructions { instructions })
}


#[derive(Debug)]
pub struct VaultStrategyInitEvent {
    pub event_id: u8,
    pub deposit_limit: u64,
}

// pub fn handle_program_log(
//     self_program_str: &str,
//     l: &str,
//     with_prefix: bool,
// ) -> Result<(Option<String>, bool), ClientError> {
//     // Log emitted from the current program.
//     if let Some(log) = if with_prefix {
//         l.strip_prefix(PROGRAM_LOG)
//             .or_else(|| l.strip_prefix(PROGRAM_DATA))
//     } else {
//         Some(l)
//     } {
//         if l.starts_with(&format!("Program log:")) {
//             // not log event
//             return Ok((None, false));
//         }
//         let borsh_bytes = match anchor_lang::__private::base64::decode(log) {
//             Ok(borsh_bytes) => borsh_bytes,
//             _ => {
//                 println!("Could not base64 decode log: {}", log);
//                 return Ok((None, false));
//             }
//         };

//         let mut slice: &[u8] = &borsh_bytes[..];
//         let disc: [u8; 8] = {
//             let mut disc = [0; 8];
//             disc.copy_from_slice(&borsh_bytes[..8]);
//             slice = &slice[8..];
//             disc
//         };
//         match disc {
//             ConfigChangeEvent::DISCRIMINATOR => {
//                 println!("{:#?}", decode_event::<ConfigChangeEvent>(&mut slice)?);
//             }
//             _ => {
//                 println!("unknow event: {}", l);
//             }
//         }
//         return Ok((None, false));
//     } else {
//         let (program, did_pop) = handle_system_log(self_program_str, l);
//         return Ok((program, did_pop));
//     }
// }

// fn decode_vault_strategy_init_event(base64_str: &str) -> Result<VaultStrategyInitEvent, Box<dyn std::error::Error>> {
//     // Decode base64 string to binary data
//     log::info!("Base64 log message before decoded_bytes: {}", base64_str);

//     let decoded_bytes = general_purpose::STANDARD.decode(base64_str)?;

//     log::info!("Base64 log message decoded_bytes: {:#?}", decoded_bytes);

//     // Ensure the length is as expected (1 byte for event_id + 8 bytes for deposit_limit)
//     if decoded_bytes.len() != 9 {
//         return Err("Invalid data length".into());
//     }

//     // Extract the `event_id` (1 byte)
//     let event_id = decoded_bytes[0];

//     // Extract the `deposit_limit` (8 bytes)
//     let deposit_limit_bytes: [u8; 8] = decoded_bytes[1..9].try_into()?;
//     let deposit_limit = u64::from_le_bytes(deposit_limit_bytes);

//     Ok(VaultStrategyInitEvent {
//         event_id,
//         deposit_limit,
//     })
// }

fn parse_filters_from_params(params: String) -> Result<InstructionFilterParams, substreams::errors::Error> {
    match serde_qs::from_str(&params) {
        Ok(filters) => Ok(filters),
        Err(e) => Err(anyhow!("Failed to parse filters from params: {}", e))
    }
}

fn apply_filter(instruction: &CompiledInstruction, filters: &InstructionFilterParams, account_keys: &Vec<&Vec<u8>>) -> bool {
    if filters.program_id.is_none() {
        return true;
    }
    let program_id_filter = filters.program_id.as_ref().unwrap();

    let program_account_key = account_keys.get(instruction.program_id_index as usize);
    if program_account_key.is_none() {
        return false;
    }
    let program_account_key_val = bs58::encode(program_account_key.unwrap()).into_string();

    if &program_account_key_val != program_id_filter {
        return false;
    }

    true
}