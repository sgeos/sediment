pub mod block;
pub mod blockchain;
pub mod hashable;
pub mod transaction;
pub mod types;
pub mod utility;

use {crate::utility::now, block::Block, blockchain::Blockchain, transaction::Transaction};

#[no_mangle]
pub extern "C" fn run() {
    let max_block = 10;
    let index = 0;
    let mut timestamp = now();
    let mut prev_block_hash = vec![0; 32];
    let user_a = "Alice".to_owned();
    let mut user_a_coins = 50;
    let user_b = "Bob".to_owned();
    let user_b_coins = 12;
    let user_c = "Chris".to_owned();
    let user_c_coins = 536;
    let difficulty = 0x00ff_ffff_ffff_ffff_ffff_ffff_ffff_ffff;

    let mut genesis_block = Block::new(
        index,
        timestamp,
        prev_block_hash,
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: user_a.clone(),
                    value: user_a_coins,
                },
                transaction::Output {
                    to_addr: user_b.clone(),
                    value: user_b_coins,
                },
            ],
        }],
        difficulty,
    );
    genesis_block.mine();
    println!("Mined Genesis Block: {genesis_block:?}");

    let mut blockchain = Blockchain::new();
    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    for i in 1..=max_block {
        timestamp = now();
        prev_block_hash = blockchain.blocks.last().unwrap().hash.clone();
        let fee = 2;
        if user_b_coins < user_a_coins {
            user_a_coins -= user_b_coins;
        } else { // overspending
            user_a_coins = 2 * fee;
        }
        user_a_coins -= fee;
        let mut block = block::Block::new(
            i,
            timestamp,
            prev_block_hash,
            vec![
                Transaction {
                    inputs: vec![],
                    outputs: vec![transaction::Output {
                        to_addr: user_c.clone(),
                        value: user_c_coins,
                    }],
                },
                Transaction {
                    inputs: vec![
                        blockchain.blocks[i as usize - 1].transactions[0].outputs[0].clone()
                    ],
                    outputs: vec![
                        transaction::Output {
                            to_addr: user_a.clone(),
                            value: user_a_coins,
                        },
                        transaction::Output {
                            to_addr: user_b.clone(),
                            value: user_b_coins,
                        },
                    ],
                },
            ],
            difficulty,
        );

        block.mine();
        println!("Mined Block {i}: {block:?}");
        blockchain
            .update_with_block(block)
            .expect(&format!("Failed to add block {i}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::BlockValidationErr::*;

    #[test]
    fn test_good_run() {
        run();
    }

    #[test]
    fn test_good_block() {
        let index = 0;
        let timestamp = now();
        let prev_block_hash = vec![0; 32];
        let user_a = "Alice".to_owned();
        let user_a_coins = 50;
        let user_b = "Bob".to_owned();
        let user_b_coins = 7;
        let difficulty = 0x00ff_ffff_ffff_ffff_ffff_ffff_ffff_ffff;

        let mut block = Block::new(
            index,
            timestamp,
            prev_block_hash,
            vec![Transaction {
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        to_addr: user_a,
                        value: user_a_coins,
                    },
                    transaction::Output {
                        to_addr: user_b,
                        value: user_b_coins,
                    },
                ],
            }],
            difficulty,
        );
        block.mine();

        let mut blockchain = Blockchain::new();
        assert!(!blockchain.update_with_block(block).is_err());
    }

    #[test]
    fn test_bad_coinbase() {
        let index = 0;
        let timestamp = now();
        let prev_block_hash = vec![0; 32];
        let user_a = "Alice".to_owned();
        let user_a_coins = 50;
        let user_b = "Bob".to_owned();
        let user_b_coins = 7;
        let user_c = "Chris".to_owned();
        let user_c_coins = 536;
        let difficulty = 0x00ff_ffff_ffff_ffff_ffff_ffff_ffff_ffff;

        let mut block = Block::new(
            index,
            timestamp,
            prev_block_hash,
            vec![Transaction {
                inputs: vec![transaction::Output {
                    to_addr: user_c,
                    value: user_c_coins,
                }],
                outputs: vec![
                    transaction::Output {
                        to_addr: user_a,
                        value: user_a_coins,
                    },
                    transaction::Output {
                        to_addr: user_b,
                        value: user_b_coins,
                    },
                ],
            }],
            difficulty,
        );
        block.mine();

        let mut blockchain = Blockchain::new();
        assert_eq!(
            blockchain.update_with_block(block),
            Err(InvalidCoinbaseTransaction)
        );
    }
}
