use blockchainlib::*;

fn main() {

let difficulty =  0x000fffffffffffffffffffffffffffff;
let mut block = Block::new(0, now(), vec![0;32], 0, "genesis block".to_owned(), difficulty );

//block.hash = block.hash();
block.mine();
println!("Mined genesis block {:?}", &block);

let mut last_hash = block.hash.clone();

let mut blockchain = Blockchain {
    blocks: vec![block],
};

//Genesis block verify
println!("Verify {}", &blockchain.verify());

for i in 1..=10 {
    let mut block = Block::new(i, now(), last_hash, 0, "Another block".to_owned(), difficulty );

    block.mine();
    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    blockchain.blocks.push(block);

    println!("Verify {}", &blockchain.verify());

}

//blockchain.blocks[3].index = 4;
//println!("Verify {}", &blockchain.verify());

}


