use sha256::digest;
use chrono::{Datelike, Utc};
//use std::io;

#[derive(Clone)]
pub struct Transaction
{
    pub from_address: i32,
    pub to_address: i32,
    pub amount: i32
}

impl Transaction
{
    pub fn new(from_address: i32, to_address: i32, amount: i32) -> Self
    {
        Transaction
        {
            from_address,
            to_address,
            amount
        }
    }
}

pub struct Block
{
    pub timestamp: String,
    pub transaction: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: i32
}

impl Block
{
    pub fn new(timestamp: &str, transaction: &Vec<Transaction>, previous_hash: &str) -> Block
    {   
        let hash = "0".to_string();//Self::calculate_hash(index, timestamp, data, previous_hash);
        
        Block 
        {  
            timestamp: timestamp.to_string(),
            transaction: transaction.to_vec(),
            previous_hash: previous_hash.to_string(),
            hash,
            nonce: 0
        }
    }

    /*fn calculate_hash(index: i32, timestamp: &str, data: i32, previous_hash: &str) -> String
    {
        let str_index = index.to_string();
        let str_data = data.to_string();

        let index = "".to_string() + &str_index + timestamp + &str_data + previous_hash;
        
        digest(index)
    }*/

    pub fn recalculate_hash(&self) -> String
    {
        let mut str_data = "".to_string();
        for i in 0..self.transaction.len()
        {
            str_data += &self.transaction[i].from_address.to_string();
            str_data += &self.transaction[i].to_address.to_string();
            str_data += &self.transaction[i].amount.to_string();
        }//self.transaction.to_string();
        let str_nonce = self.nonce.to_string();

        let index = "".to_string()  + &self.timestamp + &str_data + &self.previous_hash + &str_nonce;
        
        //self.hash = digest(index.to_string());
        digest(index)
    }

    pub fn mine_block(&mut self, difficulty: i32)
    {
        //let str;

        //let end_pos = self.hash.char_indices().nth(difficulty as usize).map(|(n, _)| n).unwrap_or(0);
        //str = &self.hash[..end_pos];
        
        //println!("{}", str);

        //let str = substring(&self.hash, 0, difficulty);
        //println!("{}", str);

        let mut temp = String::new();

        for _ in 0..difficulty
        {
            temp += "0";
        }
        //print!("Temp: {}", temp);

        while substring(&self.hash, 0, difficulty) != temp
        {
            self.nonce += 1;
            self.hash = self.recalculate_hash();
        }

        println!("Block successfully mined!: {}", self.hash);
    }
}

pub struct BlockChain
{
    pub chain: Vec<Block>,
    pub difficulty: i32,
    pub pending: Vec<Transaction>, //pending transactions
    pub mining_reward: i32
}

impl BlockChain
{
    pub fn new() -> Self
    {
        let mut chain: Vec<Block> = Vec::new();
        chain.push(Self::create_genesis_block());
        chain[0].hash = chain[0].recalculate_hash();

        BlockChain
        {
            chain,
            difficulty: 5,
            pending: Vec::new(),
            mining_reward: 100
        }
    }

    pub fn create_genesis_block() -> Block
    {
        let transaction = Vec::new();
        Block::new("01/20/2023",  &transaction, "0")
    }

    pub fn get_latest_block(&self) -> &Block
    {
        &self.chain[self.chain.len()-1]
    }

    /*fn add_block(&mut self, mut block: Block)
    {
        block.previous_hash = self.get_latest_block().hash.to_string();
        //block.hash = block.recalculate_hash();
        block.mine_block(self.difficulty);
        self.chain.push(block);
    }*/

    pub fn mine_pending_transactions(&mut self, mining_rewardaddress: i32)
    {
        let now = Utc::now();

        let year = now.year();
        let month = now.month();
        let day = now.day();

        let timestamp = "".to_string() + &month.to_string() + "/" + &day.to_string() + "/" + &year.to_string();
        
        let mut block = Block::new(&timestamp, &self.pending, &self.chain[(self.chain.len()-1) as usize].hash); //Block should not get all transaction (Like BitCOin, way to many)
        self.pending = 
        vec![
            Transaction::new(-1, mining_rewardaddress, self.mining_reward)
        ];
        block.transaction = self.pending.to_vec();
        block.mine_block(self.difficulty);
        self.chain.push(block);
    }

    pub fn create_transaction(&mut self, transaction: Transaction)
    { 
        self.pending.push(transaction);
    }

    pub fn get_balance_of_address(&mut self, address: i32) -> i32
    {
        let mut balance = 0;

        for block in self.chain.iter()
        {
            for transaction in &block.transaction
            {
                if transaction.from_address == address
                {
                    balance -= transaction.amount;
                }

                if transaction.to_address == address
                {
                    balance += transaction.amount;
                }
            }
        }

        balance
    }

    pub fn is_chain_valid(&mut self) -> bool //Very inefficient (loops over entire chain): Should only loop over the blocks next to the add/changed?
    {
        for i in 1..self.chain.len()
        {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i-1];

            if current_block.hash != current_block.recalculate_hash()
            {
                return false;
            }

            if current_block.previous_hash != previous_block.hash
            {
                return false;
            }
        }
        true
    }
}

pub fn substring(string: &str, start: i32, length: i32) -> String
{
    let mut str = String::new();
    let slice;

    //Remove beginning
    if start != 0
    {
        let mut iter = string.chars();
        iter.by_ref().nth(start as usize);

        slice = iter.as_str();
    }
    else
    {
        slice = string;
    }

    //Remove end
    let end_pos = slice.char_indices().nth(length as usize).map(|(n, _)| n).unwrap_or(0);
    let temp = &slice[..end_pos];

    str += temp;
    str
}