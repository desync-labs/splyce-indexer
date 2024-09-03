use substreams::log;
use std::error::Error;
use super::decode_data::DecodeVaultStratagyData;

use crate::{pb::sol::transactions::strategy::v1::Strategy,utils::{constants::VaultStrategyInstructions, utils::read_descriptor}};

use anchor_lang::prelude::*;


#[derive(Debug, AnchorDeserialize, AnchorSerialize)]
pub struct VaultStrategyInitEvent {
    event_id: [u8; 8],
    deposit_limit: u64,
}

impl DecodeVaultStratagyData for Strategy {
    fn parse_from_data(data: &[u8], seed: String, event_data:Vec<u8>) -> std::result::Result<Self, Box<dyn Error>> {

        log::info!("decoding vault strategy event  {:#?}",event_data);

        //Decoding event data
        let mut slice: &[u8] = &event_data[..];
        let disc: [u8; 8] = {
            let mut disc = [0; 8];
            disc.copy_from_slice(&event_data[..8]);
            slice = &slice[8..];
            disc
        };

        let event: VaultStrategyInitEvent = anchor_lang::AnchorDeserialize::deserialize(&mut slice)
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;


        log::info!("decoding vault strategy event  {:#?}",event);


        // log::info!("decoding vault strategy data  {:?}",data);
        let descriptor:&[u8] = read_descriptor(&data);
        
        let vault_strategy_instruction_type = VaultStrategyInstructions::Initialize;
    
        // Check if the descriptor matches the value of the enum variant
        if vault_strategy_instruction_type.matches(descriptor) {
            return get_vault_strategy_init_data(seed);
        } else {
            return Err("Invalid instruction type".into());
        }

        //Initilize the vault
        fn get_vault_strategy_init_data(seed: String) -> std::result::Result<Strategy, Box<dyn Error>> {
            Ok(Strategy {
                address: seed,
                underlying_mint: Vec::new(),
                underlying_token_acc: Vec::new(),
                underlying_decimals: 0,
                total_shares: 0,
                deposit_limit: 0,
                vault: Vec::new(),
                total_funds: 0,
            })
        }
    }

   
}

