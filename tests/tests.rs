use blockchain_simulation::Blockchain;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Result};
use std::time::Duration;
struct MockStdin {
    lines: Vec<String>,
    index: usize,
}

impl MockStdin {
    fn new(lines: Vec<String>) -> Self {
        MockStdin { lines, index: 0 }
    }
}

impl Read for MockStdin {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.index >= self.lines.len() {
            return Ok(0);
        }
        let line = &self.lines[self.index];
        let bytes = line.as_bytes();
        let len = bytes.len().min(buf.len());
        buf[..len].copy_from_slice(&bytes[..len]);
        self.index += 1;
        Ok(len)
    }
}

impl BufRead for MockStdin {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        if self.index >= self.lines.len() {
            return Ok(&[]);
        }
        Ok(self.lines[self.index].as_bytes())
    }

    fn consume(&mut self, amt: usize) {
        self.index += amt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockchain_simulation::storage::Storage;
    fn create_clean_blockchain() -> Blockchain {
        let clean_storage = Storage {
            accounts: HashMap::new(),
        };
        Blockchain::new(Duration::from_secs(1), Some(clean_storage))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_blockchain_simulation() {
            let commands = vec![
                "b create-account alice 1000",
                "b create-account bob 500",
                "b transfer alice bob 200",
                "b balance alice",
                "b start-node",
            ]
            .iter()
            .map(|&cmd| cmd.to_string())
            .collect::<Vec<String>>();
            let mut blockchain = create_clean_blockchain();

            let mock_stdin = MockStdin::new(commands);
            let reader = BufReader::new(mock_stdin);
            let mut output = Vec::new();
            blockchain.start_with_mocks(reader, &mut output);
        }

        #[test]
        fn test_create_account() {
            let mut blockchain = create_clean_blockchain();

            let account_name = "test_account";
            let initial_balance = 1000;

            // Check if the account exists, and create it if it doesn't
            if !blockchain.storage.accounts.contains_key(account_name) {
                blockchain
                    .create_account(account_name, initial_balance)
                    .unwrap();
                assert_eq!(
                    blockchain.storage.accounts.get(account_name).unwrap(),
                    &initial_balance
                );
            }
        }
        #[test]
        fn test_transfer_funds() {
            let mut blockchain = create_clean_blockchain();

            // Check if the accounts exist, and create them if they don't
            if !blockchain.storage.accounts.contains_key("alice") {
                blockchain.create_account("alice", 1000).unwrap();
            }
            if !blockchain.storage.accounts.contains_key("bob") {
                blockchain.create_account("bob", 500).unwrap();
            }

            blockchain.transfer("alice", "bob", 200).unwrap();
            assert_eq!(blockchain.storage.accounts.get("alice").unwrap(), &800);
            assert_eq!(blockchain.storage.accounts.get("bob").unwrap(), &700);
        }

        #[test]
        fn test_transfer_insufficient_funds() {
            let mut blockchain = create_clean_blockchain();

            blockchain.create_account("alice", 300).unwrap();
            let result = blockchain.transfer("alice", "bob", 500);
            assert!(result.is_err());
            assert_eq!(blockchain.storage.accounts.get("alice").unwrap(), &300);
            // Balance should be unchanged
        }

        #[test]
        fn test_account_not_found() {
            let blockchain = create_clean_blockchain();

            let result = blockchain.balance("nonexistent");
            assert!(result.is_err());
        }
        #[test]
        fn test_list_accounts() {
            let mut blockchain = create_clean_blockchain();
            assert!(blockchain.create_account("alice", 1000).is_ok());
            assert!(blockchain.create_account("bob", 500).is_ok());

            let accounts_list = blockchain.list_accounts();
            assert!(accounts_list.contains("Account ID: alice, Balance: 1000"));
            assert!(accounts_list.contains("Account ID: bob, Balance: 500"));
        }

        #[test]
        fn test_process_command_create_account() {
            let mut blockchain = create_clean_blockchain();

            let result = blockchain.process_command("create-account charlie 1500");
            assert!(result.is_ok());
            assert_eq!(
                result.unwrap(),
                "Account 'charlie' created with balance 1500"
            );
            assert_eq!(blockchain.storage.accounts.get("charlie").unwrap(), &1500);
        }

        #[test]
        fn test_process_command_transfer() {
            let mut blockchain = create_clean_blockchain();

            blockchain.create_account("alice", 1000).unwrap();
            blockchain.create_account("bob", 500).unwrap();

            let result = blockchain.process_command("transfer alice bob 300");
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Transferred 300 from 'alice' to 'bob'");
            assert_eq!(blockchain.storage.accounts.get("alice").unwrap(), &700);
            assert_eq!(blockchain.storage.accounts.get("bob").unwrap(), &800);
        }

        #[test]
        fn test_process_command_balance() {
            let mut blockchain = create_clean_blockchain();

            blockchain.create_account("alice", 1000).unwrap();

            let result = blockchain.process_command("balance alice");
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Balance of 'alice': 1000");
        }

        #[test]
        fn test_process_command_invalid() {
            let mut blockchain = create_clean_blockchain();

            let result = blockchain.process_command("invalid command");
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Invalid command".to_string());
        }
    }
}
