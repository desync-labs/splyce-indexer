// use std::convert::TryInto;
// use std::{convert::TryInto, error::Error};
// use crate::{pb::sol::transactions::journal::v1::JournalEntry, utils::constants::DISCRIMINATOR};

use crate::{pb::sol::transactions::vault::v1::Vault,utils::constants::DISCRIMINATOR};
use std::error::Error;
use super::decode_data::DecodeData;


//const VARINT_MAX_LEN: usize = 10; // Maximum bytes needed for varint encoded integers

// Function to read a length-prefixed bytes field
fn read_length_prefixed_bytes(data: &mut &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let length = read_varint(data)? as usize;
    if length > data.len() {
        return Err("Data too short for bytes".into());
    }
    let result = data[..length].to_vec();
    *data = &data[length..];
    Ok(result)
}

// Function to read a varint (used for lengths of repeated fields)
fn read_varint(data: &mut &[u8]) -> Result<u32, Box<dyn Error>> {
    let mut result: u32 = 0;
    let mut shift = 0;
    for byte in data.iter() {
        let b = *byte;
        result |= ((b & 0x7f) as u32) << shift;
        shift += 7;
        if b & 0x80 == 0 {
            *data = &data[1..];
            return Ok(result);
        }
        if shift >= 32 {
            return Err("Varint is too long".into());
        }
    }
    Err("Unexpected end of data".into())
}

impl DecodeData for Vault {
    fn parse_from_data(data: &[u8], hash_of_accounts:String) -> Result<Self, Box<dyn Error>> {
        //let mut dt = data;
        // Skip the first 8 bytes
        let skipped_bytes = &data[DISCRIMINATOR..];
        let mut dt = skipped_bytes;

        let bump = read_length_prefixed_bytes(&mut dt)?;
        let owner = read_length_prefixed_bytes(&mut dt)?;
        let underlying_mint = read_length_prefixed_bytes(&mut dt)?;
        let underlying_token_acc = read_length_prefixed_bytes(&mut dt)?;

        let underlying_decimals = read_varint(&mut dt)? as u32;
        let total_debt = read_varint(&mut dt)? as u64;
        let total_shares = read_varint(&mut dt)? as u64;
        let deposit_limit = read_varint(&mut dt)? as u64;
        let min_user_deposit = read_varint(&mut dt)? as u64;
        let is_shutdown = if dt[0] != 0 { true } else { false };
        dt = &dt[1..]; // Skip the bool byte

        let mut strategies = Vec::new();
        while dt.len() > 0 {
            strategies.push(read_length_prefixed_bytes(&mut dt)?);
        }

        Ok(Vault {
            bump,
            owner,
            underlying_mint,
            underlying_token_acc,
            underlying_decimals,
            total_debt,
            total_shares,
            deposit_limit,
            min_user_deposit,
            is_shutdown,
            strategies,
        })
    }
}