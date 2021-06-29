use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify (&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            //Checking the index
            if block.index != i as u32 {
                println! ("Index mismatch {} != {}",
                    &block.index,
                    &i,
                );
            return false;
            }
            //Checking difficulty match
            else if !block::check_difficulty(&block.hash(),
            block.difficulty) {
                println!("Difficulty fail");
                return false;
            }
            //The genesis block  don´t have previous hash block and 
            //don´t have to check the previous timestamp so
            //There is differenet validations
            else if i!= 0 {
                //Not genesis block
                let prev_block = &self.blocks[i -1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Time did not increase");
                    return false;
                } else if block.prev_block_hash != prev_block.hash {
                    println!("Hash mismatch");
                    return false;
                }

            } else {
                //genesis block
                if block.prev_block_hash != vec![0; 32] {
                    println!("Genesis block prev_block_hash invalid: {:?}", block.prev_block_hash);
                    return false;
                }

            }
        }

        true
    }
}