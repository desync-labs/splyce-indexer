use crate::pb::sol::{instructions::v1::{Instruction, Instructions}, transactions::v1::Transactions};
use substreams_solana::pb::sf::solana::r#type::v1::Block;

#[substreams::handlers::map]
fn filtered_instructions_without_votes(
    query: String,
    block: Block,
) -> Result<Instructions, substreams::errors::Error> {
    let query = substreams::expr_matcher(&query);

    let mut transactions = Transactions {
        transactions: block.transactions
    };

    transactions.transactions.retain(|trx| {
        trx.walk_instructions()
            .any(|view| query.matches_keys(&vec![format!("program:{}", view.program_id().to_string())]))
    });

    let instructions: Vec<Instruction> = transactions.transactions.iter_mut()
    .flat_map(|trx| {
        trx.walk_instructions().map(|view| {
            let program_id = view.program_id().to_string();
            let accounts = view.accounts().iter().map(|acct| acct.to_string()).collect();
            let data = view.data().to_vec();
    
            Instruction {
                program_id,
                accounts,
                data,
                tx_hash: "0x00".to_string(),
            }
        }).collect::<Vec<_>>()
    })
    .collect();

    Ok(Instructions { instructions })
}