pub mod block;
pub mod blockchain;
pub mod hashable;
pub mod transaction;
pub mod types;
pub mod utility;

use {block::Block, blockchain::Blockchain, transaction::Transaction, utility::now};

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
        } else {
            // overspending
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
    fn test_good_blockchain() {
        let mut index = 0;
        let mut timestamp = now();
        let mut prev_block_hash = vec![0; 32];
        let user_a = "Alice".to_owned();
        let user_a_coins = 50;
        let user_b = "Bob".to_owned();
        let user_b_coins = 7;
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

        let mut blockchain = Blockchain::new();
        blockchain
            .update_with_block(genesis_block)
            .expect("Failed to add genesis block");

        index += 1;
        timestamp = now();
        prev_block_hash = blockchain.blocks.last().unwrap().hash.clone();
        let mut block = Block::new(
            index,
            timestamp,
            prev_block_hash,
            vec![
                Transaction {
                    inputs: vec![],
                    outputs: vec![],
                },
                Transaction {
                    inputs: blockchain.blocks[0].transactions[0].outputs.clone(),
                    outputs: blockchain.blocks[0].transactions[0].outputs.clone(),
                },
            ],
            difficulty,
        );
        block.mine();

        assert!(!blockchain.update_with_block(block).is_err());
    }

    #[test]
    fn test_error_achronological_timestamp() {
        let mut index = 0;
        let mut timestamp = now();
        let mut prev_block_hash = vec![0; 32];
        let difficulty = 0x00ff_ffff_ffff_ffff_ffff_ffff_ffff_ffff;

        let mut genesis_block = Block::new(
            index,
            timestamp,
            prev_block_hash,
            vec![Transaction {
                inputs: vec![],
                outputs: vec![],
            }],
            difficulty,
        );
        genesis_block.mine();

        let mut blockchain = Blockchain::new();
        blockchain
            .update_with_block(genesis_block)
            .expect("Failed to add genesis block");

        index += 1;
        timestamp -= 1;
        prev_block_hash = blockchain.blocks.last().unwrap().hash.clone();
        let mut block = Block::new(index, timestamp, prev_block_hash, vec![], difficulty);
        block.mine();
        assert_eq!(
            blockchain.update_with_block(block),
            Err(AchronologicalTimestamp)
        );
    }

    #[test]
    fn test_error_insufficient_input_value() {
        let mut index = 0;
        let mut timestamp = now();
        let mut prev_block_hash = vec![0; 32];
        let user_a = "Alice".to_owned();
        let user_a_coins = 50;
        let difficulty = 0x00ff_ffff_ffff_ffff_ffff_ffff_ffff_ffff;

        let mut genesis_block = Block::new(
            index,
            timestamp,
            prev_block_hash,
            vec![Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: user_a.clone(),
                    value: user_a_coins,
                }],
            }],
            difficulty,
        );
        genesis_block.mine();

        let mut blockchain = Blockchain::new();
        blockchain
            .update_with_block(genesis_block)
            .expect("Failed to add genesis block");

        index += 1;
        timestamp = now();
        prev_block_hash = blockchain.blocks.last().unwrap().hash.clone();
        let mut block = Block::new(
            index,
            timestamp,
            prev_block_hash,
            vec![
                Transaction {
                    inputs: vec![],
                    outputs: vec![],
                },
                Transaction {
                    inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                    outputs: vec![transaction::Output {
                        to_addr: user_a.clone(),
                        value: user_a_coins * 2,
                    }],
                },
            ],
            difficulty,
        );
        block.mine();
        assert_eq!(
            blockchain.update_with_block(block),
            Err(InsufficientInputValue)
        );
    }

    #[test]
    fn test_error_invalid_coinbase_transaction() {
        let index = 0;
        let timestamp = now();
        let prev_block_hash = vec![0; 32];
        let user_a = "Alice".to_owned();
        let user_a_coins = 50;
        let difficulty = 0x00ff_ffff_ffff_ffff_ffff_ffff_ffff_ffff;

        let mut block = Block::new(
            index,
            timestamp,
            prev_block_hash,
            vec![Transaction {
                inputs: vec![transaction::Output {
                    to_addr: user_a,
                    value: user_a_coins,
                }],
                outputs: vec![],
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

    #[test]
    fn test_error_invalid_coinbase_transaction_fee() {
        let mut index = 0;
        let mut timestamp = now();
        let mut prev_block_hash = vec![0; 32];
        let user_a = "Alice".to_owned();
        let user_a_coins = 50;
        let user_b = "Bob".to_owned();
        let user_b_coins = 7;
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

        let mut blockchain = Blockchain::new();
        blockchain
            .update_with_block(genesis_block)
            .expect("Failed to add genesis block");

        index += 1;
        timestamp = now();
        prev_block_hash = blockchain.blocks.last().unwrap().hash.clone();
        let zero_value_transaction_vector = vec![transaction::Output {
            to_addr: user_a.clone(),
            value: 0,
        }];
        let positive_value_transaction_vector =
            vec![blockchain.blocks[0].transactions[0].outputs[0].clone()];
        let mut block = Block::new(
            index,
            timestamp,
            prev_block_hash,
            vec![
                Transaction {
                    inputs: vec![],
                    outputs: zero_value_transaction_vector.clone(),
                },
                Transaction {
                    inputs: positive_value_transaction_vector.clone(),
                    outputs: zero_value_transaction_vector.clone(),
                },
            ],
            difficulty,
        );
        block.mine();

        assert_eq!(
            blockchain.update_with_block(block),
            Err(InvalidCoinbaseTransactionFee)
        );
    }

    #[test]
    fn test_error_invalid_genesis_block_format() {
        let index = 0;
        let timestamp = now();
        let prev_block_hash = vec![0; 32];
        let user_a = "Alice".to_owned();
        let user_a_coins = 50;
        let user_b = "Bob".to_owned();
        let user_b_coins = 7;
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
        genesis_block.prev_block_hash = genesis_block.hash.clone();

        let mut blockchain = Blockchain::new();
        assert_eq!(
            blockchain.update_with_block(genesis_block),
            Err(InvalidGenesisBlockFormat)
        );
    }

    #[test]
    fn test_error_invalid_hash() {
        let mut index = 0;
        let mut timestamp = now();
        let mut prev_block_hash = vec![0; 32];
        let user_a = "Alice".to_owned();
        let user_a_coins = 50;
        let user_b = "Bob".to_owned();
        let user_b_coins = 7;
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

        let mut blockchain = Blockchain::new();
        blockchain
            .update_with_block(genesis_block)
            .expect("Failed to add genesis block");

        index += 1;
        timestamp = now();
        prev_block_hash = blockchain.blocks.last().unwrap().hash.clone();
        let mut block = Block::new(
            index,
            timestamp,
            prev_block_hash,
            vec![Transaction {
                inputs: vec![],
                outputs: vec![],
            }],
            difficulty,
        );
        block.mine();
        block.difficulty = 0;

        assert_eq!(blockchain.update_with_block(block), Err(InvalidHash));
    }

    #[test]
    fn test_error_invalid_input() {
        let mut index = 0;
        let mut timestamp = now();
        let mut prev_block_hash = vec![0; 32];
        let user_a = "Alice".to_owned();
        let user_a_coins = 50;
        let user_b = "Bob".to_owned();
        let user_b_coins = 7;
        let user_c = "Nobody".to_owned();
        let user_c_coins = 363893;
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

        let mut blockchain = Blockchain::new();
        blockchain
            .update_with_block(genesis_block)
            .expect("Failed to add genesis block");

        index += 1;
        timestamp = now();
        prev_block_hash = blockchain.blocks.last().unwrap().hash.clone();
        let mut block = Block::new(
            index,
            timestamp,
            prev_block_hash,
            vec![
                Transaction {
                    inputs: vec![],
                    outputs: vec![],
                },
                Transaction {
                    inputs: vec![
                        blockchain.blocks[0].transactions[0].outputs[0].clone(),
                        transaction::Output {
                            to_addr: user_c.clone(),
                            value: user_c_coins,
                        },
                    ],
                    outputs: vec![],
                },
            ],
            difficulty,
        );
        block.mine();

        assert_eq!(blockchain.update_with_block(block), Err(InvalidInput));
    }

    #[test]
    fn test_error_mismatched_index() {
        let index = 0;
        let mut timestamp = now();
        let mut prev_block_hash = vec![0; 32];
        let user_a = "Alice".to_owned();
        let user_a_coins = 50;
        let user_b = "Bob".to_owned();
        let user_b_coins = 7;
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

        let mut blockchain = Blockchain::new();
        blockchain
            .update_with_block(genesis_block)
            .expect("Failed to add genesis block");

        timestamp = now();
        prev_block_hash = blockchain.blocks.last().unwrap().hash.clone();
        let mut block = Block::new(
            index,
            timestamp,
            prev_block_hash,
            vec![Transaction {
                inputs: vec![],
                outputs: vec![],
            }],
            difficulty,
        );
        block.mine();
        assert_eq!(blockchain.update_with_block(block), Err(MismatchedIndex));
    }

    #[test]
    fn test_error_mismatched_previous_hash() {
        let mut index = 0;
        let mut timestamp = now();
        let mut prev_block_hash = vec![0; 32];
        let user_a = "Alice".to_owned();
        let user_a_coins = 50;
        let user_b = "Bob".to_owned();
        let user_b_coins = 7;
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

        let mut blockchain = Blockchain::new();
        blockchain
            .update_with_block(genesis_block)
            .expect("Failed to add genesis block");

        index += 1;
        timestamp = now();
        prev_block_hash = blockchain.blocks.last().unwrap().hash.clone();
        prev_block_hash[0] += 1;
        let mut block = Block::new(
            index,
            timestamp,
            prev_block_hash,
            vec![Transaction {
                inputs: vec![],
                outputs: vec![],
            }],
            difficulty,
        );
        block.mine();

        assert_eq!(
            blockchain.update_with_block(block),
            Err(MismatchedPreviousHash)
        );
    }
}
