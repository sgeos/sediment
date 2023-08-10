pub mod block;
pub mod blockchain;
pub mod hashable;

use {
    block::Block,
    blockchain::Blockchain,
    std::time::{SystemTime, UNIX_EPOCH},
};

type BlockHash = Vec<u8>;

#[no_mangle]
pub extern "C" fn run() {
    let index = 0;
    let mut timestamp = now();
    let mut previous_hash = vec![0; 32];
    let mut payload = "Genesis Block".to_owned();
    let difficulty = 0x00ff_ffff_ffff_ffff_ffff_ffff_ffff_ffff;

    let mut genesis_block = Block::new(index, timestamp, previous_hash, payload, difficulty);
    genesis_block.mine();
    println!("Mined Genesis Block: {genesis_block:?}");

    let mut blockchain = Blockchain {
        blocks: vec![genesis_block],
    };
    println!("Verify Blockchain: {}", &blockchain.verify());

    for i in 1..=10 {
        timestamp = now();
        previous_hash = blockchain.blocks.last().unwrap().hash.clone();
        payload = format!("Block {i}");
        let mut block = block::Block::new(i, timestamp, previous_hash, payload, difficulty);
        block.mine();
        println!("Mined Block {i}: {block:?}");
        blockchain.blocks.push(block);
        println!("Verify Blockchain: {}", &blockchain.verify());
    }
}

pub fn now() -> u128 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

pub fn u32_bytes(u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
    ]
}

pub fn u64_bytes(u: &u64) -> [u8; 8] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
    ]
}

pub fn u128_bytes(u: &u128) -> [u8; 16] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
        (u >> 8 * 0x8) as u8,
        (u >> 8 * 0x9) as u8,
        (u >> 8 * 0xa) as u8,
        (u >> 8 * 0xb) as u8,
        (u >> 8 * 0xc) as u8,
        (u >> 8 * 0xd) as u8,
        (u >> 8 * 0xe) as u8,
        (u >> 8 * 0xf) as u8,
    ]
}

pub fn difficulty_bytes_as_u128(v: &Vec<u8>) -> u128 {
    ((v[31] as u128) << 0xf * 8)
        | ((v[30] as u128) << 0xe * 8)
        | ((v[29] as u128) << 0xd * 8)
        | ((v[28] as u128) << 0xc * 8)
        | ((v[27] as u128) << 0xb * 8)
        | ((v[26] as u128) << 0xa * 8)
        | ((v[25] as u128) << 0x9 * 8)
        | ((v[24] as u128) << 0x8 * 8)
        | ((v[23] as u128) << 0x7 * 8)
        | ((v[22] as u128) << 0x6 * 8)
        | ((v[21] as u128) << 0x5 * 8)
        | ((v[20] as u128) << 0x4 * 8)
        | ((v[19] as u128) << 0x3 * 8)
        | ((v[18] as u128) << 0x2 * 8)
        | ((v[17] as u128) << 0x1 * 8)
        | ((v[16] as u128) << 0x0 * 8)
}

#[cfg(test)]
mod tests {
    use super::*;

fn test_blockchain(length: u32) -> Blockchain {
    let index = 0;
    let mut timestamp = now();
    let mut previous_hash = vec![0; 32];
    let mut payload = "Genesis Block".to_owned();
    let difficulty = 0x00ff_ffff_ffff_ffff_ffff_ffff_ffff_ffff;

    let mut genesis_block = Block::new(index, timestamp, previous_hash, payload, difficulty);
    genesis_block.mine();

    let mut blockchain = Blockchain {
        blocks: vec![genesis_block],
    };

    for i in 1..=length {
        timestamp = now();
        previous_hash = blockchain.blocks.last().unwrap().hash.clone();
        payload = format!("Block {i}");
        let mut block = block::Block::new(i, timestamp, previous_hash, payload, difficulty);
        block.mine();
        blockchain.blocks.push(block);
    }

    blockchain
}

    #[test]
    fn runs_ok() {
        run();
        let blockchain = test_blockchain(10);
        assert_eq!(true, blockchain.verify());
    }

    #[test]
    fn test_bad_index() {
        let mut blockchain = test_blockchain(10);
        blockchain.blocks[3].index = 4;
        assert_eq!(false, blockchain.verify());
    }

    #[test]
    fn test_bad_hash() {
        let mut blockchain = test_blockchain(10);
        blockchain.blocks[3].hash[8] += 1;
        assert_eq!(false, blockchain.verify());
    }

    #[test]
    fn test_bad_prev_hash() {
        let mut blockchain = test_blockchain(10);
        blockchain.blocks[3].prev_block_hash[8] += 1;
        assert_eq!(false, blockchain.verify());
    }

    #[test]
    fn test_bad_genesis_prev_hash() {
        let mut blockchain = test_blockchain(10);
        blockchain.blocks[0].prev_block_hash[8] += 1;
        assert_eq!(false, blockchain.verify());
    }

    #[test]
    fn test_bad_difficulty() {
        let mut blockchain = test_blockchain(10);
        blockchain.blocks[3].difficulty = 0;
        assert_eq!(false, blockchain.verify());
    }

    #[test]
    fn test_bad_timestamp() {
        let mut blockchain = test_blockchain(10);
        blockchain.blocks[3].timestamp = now();
        assert_eq!(false, blockchain.verify());
    }
}
