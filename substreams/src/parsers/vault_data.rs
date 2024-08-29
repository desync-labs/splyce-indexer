// use std::convert::TryInto;
// use std::{convert::TryInto, error::Error};
// use crate::{pb::sol::transactions::journal::v1::JournalEntry, utils::constants::DISCRIMINATOR};

use substreams::log;

use crate::{pb::sol::transactions::vault::v1::Vault,utils::{constants::{InstructionType, DISCRIMINATOR}, utils::read_descriptor}};
use std::error::Error;
use super::decode_data::{DecodeData, DecodeVaultData};


//const VARINT_MAX_LEN: usize = 10; // Maximum bytes needed for varint encoded integers

// // Function to read a length-prefixed bytes field
// fn read_length_prefixed_bytes(data: &mut &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
//     let length = read_varint(data)? as usize;
//     if length > data.len() {
//         return Err("Data too short for bytes".into());
//     }
//     let result = data[..length].to_vec();
//     *data = &data[length..];
//     Ok(result)
// }

// // Function to read a varint (used for lengths of repeated fields)
// fn read_varint(data: &mut &[u8]) -> Result<u32, Box<dyn Error>> {
//     let mut result: u32 = 0;
//     let mut shift = 0;
//     for byte in data.iter() {
//         let b = *byte;
//         result |= ((b & 0x7f) as u32) << shift;
//         shift += 7;
//         if b & 0x80 == 0 {
//             *data = &data[1..];
//             return Ok(result);
//         }
//         if shift >= 32 {
//             return Err("Varint is too long".into());
//         }
//     }
//     Err("Unexpected end of data".into())
// }

// impl DecodeData for Vault {
//     fn parse_from_data(data: &[u8], hash_of_accounts:String) -> Result<Self, Box<dyn Error>> {
//         // Skip the first 8 bytes
//         let skipped_bytes = &data[DISCRIMINATOR..];
//         let mut dt = skipped_bytes;

//         log::info!("data: {:?}", data);

//         let bump = read_length_prefixed_bytes(&mut dt)?;

//         log::info!("bump: {:?}", bump);

//         let owner = read_length_prefixed_bytes(&mut dt)?;
//         let underlying_mint = read_length_prefixed_bytes(&mut dt)?;
//         let underlying_token_acc = read_length_prefixed_bytes(&mut dt)?;

//         let underlying_decimals = read_varint(&mut dt)? as u32;
//         let total_debt = read_varint(&mut dt)? as u64;
//         let total_shares = read_varint(&mut dt)? as u64;
//         let deposit_limit = read_varint(&mut dt)? as u64;
//         let min_user_deposit = read_varint(&mut dt)? as u64;
//         let is_shutdown = if dt[0] != 0 { true } else { false };
//         dt = &dt[1..]; // Skip the bool byte

//         let mut strategies = Vec::new();
//         while dt.len() > 0 {
//             strategies.push(read_length_prefixed_bytes(&mut dt)?);
//         }

//         Ok(Vault {
//             bump,
//             owner,
//             underlying_mint,
//             underlying_token_acc,
//             underlying_decimals,
//             total_debt,
//             total_shares,
//             deposit_limit,
//             min_user_deposit,
//             is_shutdown,
//             strategies,
//         })
//     }
// }

impl DecodeVaultData for Vault {
    fn parse_from_data(data: &[u8], seed: String) -> Result<Self, Box<dyn Error>> {
        let descriptor:&[u8] = read_descriptor(&data);
        
        let vault_instruction_type = InstructionType::VaultInit;
    
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

