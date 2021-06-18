use blockchainlib::*;

fn main() {

let mut block = Block::new(13, now(), vec![0,32], 0, "genesis block 2".to_owned(), 0x000000ffffffffffffffffffffffffff);

block.hash = block.hash();

//println!("{:?}", &block);

block.mine();

println!("{:?}", &block);

}


