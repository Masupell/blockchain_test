pub mod blockchain;
//pub mod keygenerator;

use crate::blockchain::{BlockChain, Transaction};
//use crate::blockchain::Block;
//use crate::blockchain::Transaction;

fn main() 
{
    let mut block_chain = BlockChain::new();

    println!("\nStarting miner...");
    block_chain.mine_pending_transactions(0);

    println!("\nYour Balance is {}", block_chain.get_balance_of_address(0));

    println!("\nStarting second miner...");
    block_chain.mine_pending_transactions(0);

    println!("\nYour Balance is {}", block_chain.get_balance_of_address(0));
}