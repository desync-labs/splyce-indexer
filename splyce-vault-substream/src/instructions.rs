use crate::pb::sol::instructions::v1::{Instruction, Instructions};
use substreams_solana::pb::sf::solana::r#type::v1::Block;

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
        // Collect instructions and filter based on query
        let instructions: Vec<Instruction> = trx.walk_instructions()
            .filter_map(|view| {
                let program_id = view.program_id().to_string();
                let accounts = view.accounts().iter().map(|acct| acct.to_string()).collect();
                let data = view.data().to_vec();

                if query.matches_keys(&vec![format!("program:{}", program_id.clone())]) {
                    Some(Instruction {
                        program_id,
                        accounts,
                        data,
                        tx_hash: "0x00".to_string(),
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