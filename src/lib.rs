// use std::collections::HashMap;
use crate::storage::Storage;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
pub mod storage;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Transaction {
    Transfer {
        from: String,
        to: String,
        amount: u64,
    },
    CreateAccount {
        id: String,
        balance: u64,
    },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    timestamp: u64,
    transactions: Vec<Transaction>,
    prev_block_hash: String,
    nonce: u64,
}

impl Block {
    fn new(transactions: Vec<Transaction>, prev_block_hash: String) -> Self {
        Block {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            transactions,
            prev_block_hash,
            nonce: 0, // This would be set by the mining process
        }
    }
}

pub struct Blockchain {
    pub storage: Storage,
    pub pending_transactions: Vec<Transaction>,
    pub block_time: Duration,
    pub chain: Vec<Block>,
    pub is_active: bool,
}

impl Blockchain {
    pub fn new(block_time: Duration) -> Self {
        let genesis_block = Block::new(vec![], "".to_string()); // Create the genesis block with no transactions
        Blockchain {
            storage: Storage::load().unwrap_or_default(),
            pending_transactions: Vec::new(),
            block_time,
            chain: vec![genesis_block],
            is_active: false,
        }
    }

    pub fn start_node(blockchain: Arc<Mutex<Blockchain>>) {
        let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
        println!("Starting blockchain node on TCP port 3000...");

        // Set the node to active when starting
        {
            let mut bc = blockchain.lock().unwrap();
            bc.is_active = true;
        }

        let blockchain_clone1 = blockchain.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let blockchain_clone = blockchain_clone1.clone();
                        Self::handle_connection(stream, blockchain_clone);
                    }
                    Err(_e) => { /* handle error */ }
                }
            }
        });

        let block_time = {
            let bc = blockchain.lock().unwrap();
            bc.block_time
        }; // Release the lock here

        let blockchain_clone2 = blockchain.clone();
        std::thread::spawn(move || {
            let blockchain_clone = blockchain_clone2.clone();
            loop {
                {
                    let mut bc = blockchain_clone.lock().unwrap();
                    if !bc.is_active {
                        break;
                    }
                    bc.mine_block();
                }
                std::thread::sleep(block_time);
            }
        });
    }
    fn handle_connection(mut stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
        let mut buffer = Vec::new();
        let mut stream_clone = stream.try_clone().expect("Failed to clone stream");

        match stream_clone.read_to_end(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    println!("No data received, closing connection");
                    return;
                }
                let command = String::from_utf8_lossy(&buffer).trim().to_string();
                println!("Received command: '{}'", command);
                let mut bc = blockchain.lock().unwrap();
                match bc.process_command(&command) {
                    Ok(message) => println!("{}", message),
                    Err(error) => println!("Error: {}", error),
                }
            }
            Err(e) => {
                println!(
                    "An error occurred, terminating connection with {}: {}",
                    stream.peer_addr().unwrap(),
                    e
                );
                return;
            }
        }
    }

    pub fn stop_node(&mut self) {
        self.is_active = false;
        println!("Blockchain node stopped.");
    }

    pub fn add_transaction(&mut self, from: String, to: String, amount: u64) -> Result<(), String> {
        // Check if the from account exists and has enough balance
        let from_balance = self.storage.accounts.get(&from).copied().unwrap_or(0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        // Create the transaction and add it to the pending transactions list
        let transaction = Transaction::Transfer { from, to, amount };
        self.pending_transactions.push(transaction);
        Ok(())
    }

    pub fn mine_block(&mut self) {
        if !self.is_active {
            println!("Mining attempted while blockchain node is inactive.");
            return;
        }

        if self.pending_transactions.is_empty() {
            println!("No transactions to mine, waiting for new transactions...");
            return;
        }

        println!("Starting to mine a new block...");
        let transactions = std::mem::take(&mut self.pending_transactions);
        let prev_block_hash = self.get_last_block_hash();
        let new_block = Block::new(transactions.clone(), prev_block_hash);

        for transaction in &transactions {
            match transaction {
                Transaction::CreateAccount { id, balance } => {
                    println!("Processing create-account transaction for '{}'", id);
                    self.storage.accounts.insert(id.clone(), *balance);
                }
                Transaction::Transfer { from, to, amount } => {
                    println!(
                        "Processing transfer transaction from '{}' to '{}' for amount {}",
                        from, to, amount
                    );
                    // Here you should update balances or confirm the transaction
                }
            }
        }

        self.chain.push(new_block.clone());
        self.storage.save().expect("Failed to save storage");
        println!(
            "Block mined and added to the chain with hash: {}, containing {} transactions",
            new_block.prev_block_hash, // Assuming this should be the new block's hash
            transactions.len()
        );
    }
    fn get_last_block_hash(&self) -> String {
        if let Some(last_block) = self.chain.last() {
            // In a real blockchain, this would be a cryptographic hash
            last_block.timestamp.to_string() + &last_block.prev_block_hash
        } else {
            "".to_string()
        }
    }

    pub fn create_account(&mut self, id: &str, balance: u64) -> Result<(), String> {
        if !self.is_active {
            return Err("Blockchain node is not running".to_string());
        }

        if self.storage.accounts.contains_key(id) {
            return Err("Account already exists".to_string());
        }

        let transaction = Transaction::CreateAccount {
            id: id.to_string(),
            balance,
        };
        self.pending_transactions.push(transaction);

        Ok(())
    }

    pub fn list_accounts(&self) -> String {
        if self.storage.accounts.is_empty() {
            return "No accounts found.".to_string();
        }
        let mut accounts_list = String::new();
        for (id, balance) in &self.storage.accounts {
            accounts_list.push_str(&format!("Account ID: {}, Balance: {}\n", id, balance));
        }
        accounts_list
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        let from_balance = self
            .storage
            .accounts
            .get_mut(from)
            .ok_or_else(|| "From account not found".to_string())?;
        if *from_balance < amount {
            return Err("Insufficient funds".to_string());
        }

        *from_balance -= amount;
        let to_balance = self.storage.accounts.entry(to.to_string()).or_insert(0);
        *to_balance += amount;

        // Save the updated storage
        self.storage.save()?;

        // Create a transfer transaction and add it to the pending transactions list
        let transaction = Transaction::Transfer {
            from: from.to_string(),
            to: to.to_string(),
            amount,
        };
        self.pending_transactions.push(transaction);

        Ok(())
    }

    pub fn balance(&self, account: &str) -> Result<u64, String> {
        self.storage
            .accounts
            .get(account)
            .copied()
            .ok_or_else(|| "Account not found".to_string())
    }

    pub fn start_with_mocks<R: BufRead, W: Write>(&mut self, reader: R, mut writer: W) {
        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(e) => {
                    writeln!(writer, "Error reading line: {}", e).unwrap();
                    continue;
                }
            };

            match self.process_command(&line) {
                Ok(message) => writeln!(writer, "{}", message).unwrap(),
                Err(e) => writeln!(writer, "Error: {}", e).unwrap(),
            }
        }
    }

    pub fn process_command(&mut self, command: &str) -> Result<String, String> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        match parts.as_slice() {
            ["create-account", id, balance] => {
                let balance = balance
                    .parse::<u64>()
                    .map_err(|_| "Invalid balance".to_string())?;
                self.create_account(id, balance)
                    .map(|_| format!("Account '{}' created with balance {}", id, balance))
            }
            ["transfer", from, to, amount] => {
                let amount = amount
                    .parse::<u64>()
                    .map_err(|_| "Invalid amount".to_string())?;
                self.transfer(from, to, amount)
                    .map(|_| format!("Transferred {} from '{}' to '{}'", amount, from, to))
            }
            ["balance", account] => self
                .balance(account)
                .map(|balance| format!("Balance of '{}': {}", account, balance)),
            _ => Err("Invalid command".to_string()),
        }
    }
}
