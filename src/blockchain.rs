use super::block::{check_difficulty, Block};

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            let index = block.index;
            if i as u32 != block.index {
                println!("Index mismatch {index} != {i};");
                return false;
            } else if !check_difficulty(&block.hash, block.difficulty) {
                println!("Difficulty fail.");
                return false;
            } else if 0 == i && vec![0; 32] != block.prev_block_hash {
                println!("Genesis block previous_block_hash invalid.");
                return false;
            } else if 0 < i {
                let prev_block = &self.blocks[i - 1];
                if block.timestamp < prev_block.timestamp {
                    println!("Timestamp out of order.");
                    return false;
                } else if block.prev_block_hash != prev_block.hash {
                    println!("Hash mismatch.");
                    return false;
                }
            }
        }
        true
    }
}
