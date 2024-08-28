use substreams::log;

use crate::pb::sol::transactions::{vault::v1::{Vaults, Vault}, v1::Instructions};
use crate::parsers::decode_data::DecodeData;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::error::Error;

#[substreams::handlers::map]
fn map_parse_vault_data(insts: Instructions) -> Result<Vaults, substreams::errors::Error> {
    log::info!("Parsing vault data");

    let mut transaction_data = Vec::new();

    for inst in insts.instructions.iter() {
        let hash_of_accounts = create_unique_hash_from_accounts(inst.accounts.clone());

        match decode_and_parse_to_protobuf::<Vault>(inst.data.clone(), hash_of_accounts) {
            Ok(parsed_message) => transaction_data.push(parsed_message),
            Err(e) => {
                // Log the error and continue with an empty Vault
                log::info!("Failed to decode data: {}", e);
                transaction_data.push(Vault {
                    bump: Vec::new(),
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
                });
            }
        }
    }

    Ok(Vaults { vaults: transaction_data })
}
// fn map_parse_vault_data(insts: Instructions) -> Result<Vaults, substreams::errors::Error> {
//     log::info!("Parsing vault data");


    
//     let transaction_data: Vec<Vault> = insts.instructions.iter()
//         .map(|inst| {
//             // Assuming inst.data contains the Base58 encoded string
//             let hash_of_accounts = create_unique_hash_from_accounts(inst.accounts.clone()); 
//             match decode_and_parse_to_protobuf::<Vault>(inst.data.clone(),hash_of_accounts) {
//                 Ok(parsed_message) => {
//                     Ok(parsed_message)
//                 },
//                 Err(e) => {
//                     Err(Box::new(e))
//                 }
//             }
//         })
//         .filter_map(Result::ok)
//         .collect::<Vec<_>>();

//     Ok(Vaults { vaults: transaction_data })
// }

fn create_unique_hash_from_accounts(accounts: Vec<String>) -> String {
    let concatenated_accounts: String = accounts.iter()
    .map(|acct| acct.to_string()) // Convert each account ID to a string
    .collect::<Vec<String>>()
    .join(""); // Join all strings without any separator

    // Compute the hash of the concatenated string
    let mut hasher: DefaultHasher = DefaultHasher::new();
    concatenated_accounts.hash(&mut hasher);
    let hash = hasher.finish();
    return hash.to_string();
}

fn decode_and_parse_to_protobuf<T: DecodeData>(data: Vec<u8>, hash_of_accounts:String) -> Result<T, Box<dyn Error>> {
    T::parse_from_data(&data, hash_of_accounts)
}