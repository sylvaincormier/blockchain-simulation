// Assuming this is in `blockchain.rs` inside the `blockchain` module
use std::collections::HashMap;
use std::time::Duration;

pub struct Blockchain {
    pub accounts: HashMap<String, u64>,
    pub pending_transactions: Vec<Transaction>,
    pub block_time: Duration,
}

enum Transaction {
    Transfer { from: String, to: String, amount: u64 },
}

impl Blockchain {
    pub fn new(block_time: Duration) -> Self {
        Blockchain {
            accounts: HashMap::new(),
            pending_transactions: Vec::new(),
            block_time,
        }
    }

    pub fn create_account(&mut self, id: &str, balance: u64) -> Result<(), String> {
        if self.accounts.contains_key(id) {
            return Err("Account already exists".to_string());
        }
        self.accounts.insert(id.to_string(), balance);
        Ok(())
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        let from_balance = self.accounts.get_mut(from).ok_or("From account not found")?;
        if *from_balance < amount {
            return Err("Insufficient funds".into());
        }
        *from_balance -= amount;
        let to_balance = self.accounts.entry(to.to_string()).or_insert(0);
        *to_balance += amount;
        Ok(())
    }

    pub fn balance(&self, account: &str) -> Result<u64, String> {
        self.accounts.get(account).copied().ok_or_else(|| "Account not found".to_string())
    }

    pub fn mine_block(&mut self) {
        println!("Mining new block...");
        // Here you would handle the transactions and add them to a block
        self.pending_transactions.clear();
    }
}
