use substreams::log;
use std::error::Error;
use crate::{parsers::decode_data::DecodeVaultStratagyData, pb::sol::transactions::{strategy::v1::{Strategies, Strategy}, v1::Instructions}};

#[substreams::handlers::map]
fn map_parse_vault_strategy_data(insts: Instructions) -> Result<Strategies, substreams::errors::Error> {
    log::info!("Parsing vault strategy data");

    let mut transaction_data = Vec::new();

    for inst in insts.instructions.iter() {
        match decode_and_parse_to_protobuf::<Strategy>(inst.data.clone(), inst.accounts[0].clone(), inst.event_log.clone()) {
            Ok(parsed_message) => transaction_data.push(parsed_message),
            Err(e) => {
                // Log the error and continue with an empty Vault
                log::info!("Failed to decode data: {}", e);
                
            }
        }
    }

    Ok(Strategies { strategies: transaction_data })
}

fn decode_and_parse_to_protobuf<T: DecodeVaultStratagyData>(data: Vec<u8>, seed: String, event_data:Vec<u8>) -> Result<T, Box<dyn Error>> {
    T::parse_from_data(&data,seed,event_data)
}