use substreams::log;

use crate::{parsers::decode_data::DecodeVaultData, pb::sol::transactions::{v1::Instructions, vault::v1::{Vault, Vaults}}};
use std::error::Error;

#[substreams::handlers::map]
fn map_parse_vault_data(insts: Instructions) -> Result<Vaults, substreams::errors::Error> {
    log::info!("Parsing vault data");

    let mut transaction_data = Vec::new();

    for inst in insts.instructions.iter() {
        match decode_and_parse_to_protobuf::<Vault>(inst.data.clone(), inst.accounts[0].clone()) {
            Ok(parsed_message) => transaction_data.push(parsed_message),
            Err(e) => {
                // Log the error and continue with an empty Vault
                log::info!("Failed to decode data: {}", e);
                
            }
        }
    }

    Ok(Vaults { vaults: transaction_data })
}

fn decode_and_parse_to_protobuf<T: DecodeVaultData>(data: Vec<u8>, seed: String) -> Result<T, Box<dyn Error>> {
    T::parse_from_data(&data,seed)
}