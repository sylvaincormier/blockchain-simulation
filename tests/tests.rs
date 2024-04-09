use blockchain_simulation::Blockchain;
use blockchain_simulation::Transaction;
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
    use blockchain_simulation::Blockchain;
    fn create_clean_blockchain() -> Blockchain {
        let clean_storage = Storage {
            accounts: HashMap::new(),
        };
        let mut blockchain = Blockchain::new(Duration::from_secs(1));
        blockchain.is_active = true; // Make sure the blockchain is active
        blockchain.storage = clean_storage;
        blockchain
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
            let initial_balance = 1000u64;

            blockchain
                .create_account(account_name, initial_balance)
                .unwrap();

            // Verify the transaction is in pending_transactions
            assert!(blockchain.pending_transactions.iter().any(|tx| match tx {
                Transaction::CreateAccount { id, balance } =>
                    id == account_name && *balance == initial_balance,
                _ => false,
            }));

            // Process transactions through mining to reflect in storage
            blockchain.mine_block();

            // Check that the account is created in storage
            assert_eq!(
                blockchain.storage.accounts.get(account_name),
                Some(&initial_balance)
            );
        }

        #[test]
        fn test_transfer_funds() {
            let mut blockchain = create_clean_blockchain();

            // Create accounts with initial balances
            assert!(blockchain.create_account("alice", 1000).is_ok());
            assert!(blockchain.create_account("bob", 500).is_ok());

            // Mine a block to process account creation transactions
            blockchain.mine_block();

            // Ensure the initial balances are set correctly
            assert_eq!(blockchain.balance("alice").unwrap(), 1000);
            assert_eq!(blockchain.balance("bob").unwrap(), 500);

            // Perform the transfer
            assert!(blockchain.transfer("alice", "bob", 200).is_ok());

            // Mine another block to process the transfer transaction
            blockchain.mine_block();

            // Check the balances after the transfer
            assert_eq!(
                blockchain.balance("alice").unwrap(),
                800,
                "Alice's balance should be 800 after the transfer"
            );
            assert_eq!(
                blockchain.balance("bob").unwrap(),
                700,
                "Bob's balance should be 700 after the transfer"
            );
        }

        #[test]
        fn test_transfer_insufficient_funds() {
            let mut blockchain = create_clean_blockchain();

            blockchain.create_account("alice", 300).unwrap();

            // Process the creation transaction
            blockchain.mine_block();

            let result = blockchain.transfer("alice", "bob", 500);
            assert!(
                result.is_err(),
                "Transfer should fail due to insufficient funds"
            );

            assert_eq!(
                blockchain.storage.accounts.get("alice").unwrap(),
                &300,
                "Alice's balance should remain unchanged"
            );
        }

        #[test]
        fn test_account_not_found() {
            let blockchain = create_clean_blockchain();

            let result = blockchain.balance("nonexistent");
            assert!(
                result.is_err(),
                "Querying a nonexistent account should result in an error"
            );
        }

        #[test]
        fn test_list_accounts() {
            let mut blockchain = create_clean_blockchain();
            blockchain.create_account("alice", 1000).unwrap();
            blockchain.create_account("bob", 500).unwrap();

            // Process the creation transactions
            blockchain.mine_block();

            let accounts_list = blockchain.list_accounts();
            assert!(
                accounts_list.contains("Account ID: alice, Balance: 1000"),
                "Alice should be listed with a balance of 1000"
            );
            assert!(
                accounts_list.contains("Account ID: bob, Balance: 500"),
                "Bob should be listed with a balance of 500"
            );
        }

        #[test]
        fn test_process_command_create_account() {
            let mut blockchain = create_clean_blockchain();

            let result = blockchain.process_command("create-account charlie 1500");
            assert!(result.is_ok(), "Creating an account should succeed");

            // Process the creation transaction
            blockchain.mine_block();

            assert_eq!(
                blockchain.storage.accounts.get("charlie").unwrap(),
                &1500,
                "Charlie's account should be created with a balance of 1500"
            );
        }

        #[test]
        fn test_process_command_balance() {
            let mut blockchain = create_clean_blockchain();

            // Create an account and process its creation to update the storage
            let account_name = "alice";
            let initial_balance = 1000u64;
            blockchain
                .create_account(account_name, initial_balance)
                .unwrap();
            blockchain.mine_block(); // Simulate mining to process transactions

            // Test the balance command
            let result = blockchain.process_command(&format!("balance {}", account_name));
            assert!(result.is_ok(), "Balance command should succeed");
            assert_eq!(
                result.unwrap(),
                format!("Balance of '{}': {}", account_name, initial_balance),
                "Balance should be correctly retrieved"
            );
        }
        #[test]
        fn test_process_command_transfer() {
            let mut blockchain = create_clean_blockchain();

            blockchain.create_account("alice", 1000).unwrap();
            blockchain.create_account("bob", 500).unwrap();
            blockchain.mine_block(); // Mine to process account creations

            blockchain
                .process_command("transfer alice bob 200")
                .unwrap();
            blockchain.mine_block(); // Mine to process the transfer

            assert_eq!(
                blockchain.storage.accounts.get("alice").unwrap(),
                &800,
                "Alice's balance should be 800 after transfer"
            );
            assert_eq!(
                blockchain.storage.accounts.get("bob").unwrap(),
                &700,
                "Bob's balance should be 700 after transfer"
            );
        }

        #[test]
        fn test_blockchain_operation() {
            let mut blockchain = create_clean_blockchain();
            blockchain.stop_node();
            // Verify that the blockchain node is inactive initially
            assert!(!blockchain.is_active, "Node should initially be inactive");

            // // Start the blockchain node
            blockchain.is_active = true;
            assert!(blockchain.is_active, "Node should be active after starting");

            // Create accounts
            blockchain.create_account("alice", 1000).unwrap();
            blockchain.create_account("bob", 500).unwrap();

            // Transactions should be pending, not yet affecting the storage
            assert!(
                !blockchain.storage.accounts.contains_key("alice"),
                "Alice's account should not exist until transactions are mined"
            );
            assert!(
                !blockchain.storage.accounts.contains_key("bob"),
                "Bob's account should not exist until transactions are mined"
            );

            // Mine a block to process pending transactions
            blockchain.mine_block();

            // Now the accounts should be in storage with the correct balances
            assert_eq!(
                blockchain.storage.accounts.get("alice"),
                Some(&1000),
                "Alice's account should exist with correct balance after mining"
            );
            assert_eq!(
                blockchain.storage.accounts.get("bob"),
                Some(&500),
                "Bob's account should exist with correct balance after mining"
            );
        }
    }
}
