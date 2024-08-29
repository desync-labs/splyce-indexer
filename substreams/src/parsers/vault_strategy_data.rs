use substreams::log;

use crate::{pb::sol::transactions::vault::v1::Vault,utils::{constants::{VaultInstructions, VaultStrategyInstructions, DISCRIMINATOR}, utils::read_descriptor}};
use std::error::Error;
use super::decode_data::{DecodeData, DecodeVaultData, DecodeVaultStratagyData};

impl DecodeVaultStratagyData for Vault {
    fn parse_from_data(data: &[u8], seed: String) -> Result<Self, Box<dyn Error>> {
        log::info!("decoding vault strategy data  {:?}",data);

        let descriptor:&[u8] = read_descriptor(&data);
        
        let vault_instruction_type = VaultStrategyInstructions::Initialize;
    
        // Check if the descriptor matches the value of the enum variant
        if vault_instruction_type.matches(descriptor) {
            return get_vault_init_data(seed);
        } else {
            return Err("Invalid instruction type".into());
        }

        //Initilize the vault
        fn get_vault_init_data(seed: String) -> Result<Vault, Box<dyn Error>> {
            Ok(Vault {
                address: seed,
                owner: Vec::new(),
                underlying_mint: Vec::new(),
                underlying_token_acc: Vec::new(),
                underlying_decimals: 0,
                total_debt: 0,
                total_shares: 0,
                deposit_limit: 0,
                min_user_deposit: 0,
                is_shutdown: false,
                strategies: Vec::new(),
            })
        }
    }

   
}

