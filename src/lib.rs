pub mod storage;
use crate::storage::Storage;
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::time::Duration;

pub struct Blockchain {
    pub storage: Storage,
    pub pending_transactions: Vec<Transaction>,
    pub block_time: Duration,
}

pub enum Transaction {
    Transfer {
        from: String,
        to: String,
        amount: u64,
    },
}

impl Blockchain {
    pub fn new(block_time: Duration, storage: Option<Storage>) -> Self {
        let storage = storage.unwrap_or_else(|| {
            Storage::load().unwrap_or_else(|_| Storage {
                accounts: HashMap::new(),
            })
        });
        Blockchain {
            storage,
            pending_transactions: Vec::new(),
            block_time,
        }
    }

    pub fn create_account(&mut self, id: &str, balance: u64) -> Result<(), String> {
        if self.storage.accounts.contains_key(id) {
            return Err("Account already exists".to_string());
        }
        self.storage.accounts.insert(id.to_string(), balance);
        self.storage.save()?;
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
            return Err("Insufficient funds".into());
        }
        *from_balance -= amount;
        let to_balance = self.storage.accounts.entry(to.to_string()).or_insert(0);
        *to_balance += amount;
        self.storage.save()?;
        Ok(())
    }

    pub fn balance(&self, account: &str) -> Result<u64, String> {
        self.storage
            .accounts
            .get(account)
            .copied()
            .ok_or_else(|| "Account not found".to_string())
    }

    pub fn mine_block(&mut self) {
        println!("Mining new block...");
        self.pending_transactions.clear();
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
