use blockchain_simulation::blockchain::Blockchain;
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

        let mut blockchain = Blockchain::new(Duration::from_secs(10));
        let mock_stdin = MockStdin::new(commands);
        let reader = BufReader::new(mock_stdin);
        let mut output = Vec::new();
        blockchain.start_with_mocks(reader, &mut output);
    }
    #[test]
    fn test_create_account() {
        let mut blockchain = Blockchain::new(Duration::from_secs(1));
        let account_name = "test_account";
        let initial_balance = 1000;
        blockchain
            .create_account(account_name, initial_balance)
            .unwrap();

        assert_eq!(
            blockchain.accounts.get(account_name).unwrap(),
            &initial_balance
        );
    }
    #[test]
fn test_transfer_funds() {
    let mut blockchain = Blockchain::new(Duration::from_secs(1));
    blockchain.create_account("alice", 1000).unwrap();
    blockchain.create_account("bob", 500).unwrap();

    blockchain.transfer("alice", "bob", 200).unwrap();

    assert_eq!(blockchain.accounts.get("alice").unwrap(), &800);
    assert_eq!(blockchain.accounts.get("bob").unwrap(), &700);
}
#[test]
fn test_transfer_insufficient_funds() {
    let mut blockchain = Blockchain::new(Duration::from_secs(1));
    blockchain.create_account("alice", 300).unwrap();

    let result = blockchain.transfer("alice", "bob", 500);

    assert!(result.is_err());
    assert_eq!(blockchain.accounts.get("alice").unwrap(), &300);  // Balance should be unchanged
}
#[test]
fn test_account_not_found() {
    let mut blockchain = Blockchain::new(Duration::from_secs(1));

    let result = blockchain.balance("nonexistent");

    assert!(result.is_err());
}
#[test]
fn test_mine_block() {
    let mut blockchain = Blockchain::new(Duration::from_millis(10));
    
    // Ensure "alice" and "bob" accounts are created before transferring and mining
    blockchain.create_account("alice", 1000).unwrap();
    blockchain.create_account("bob", 500).unwrap();

    // Perform a transfer to create a pending transaction
    blockchain.transfer("alice", "bob", 50).unwrap();

    // Now mine the block
    blockchain.mine_block();

    // After mining, the pending transactions should be cleared
    assert!(blockchain.pending_transactions.is_empty(), "Pending transactions should be cleared after mining a block");

    // Optionally, check the balances to ensure they are updated correctly
    assert_eq!(blockchain.accounts.get("alice").unwrap(), &(1000 - 50));
    assert_eq!(blockchain.accounts.get("bob").unwrap(), &(500 + 50));
}


fn test_start_with_mocks() {
    let commands = vec![
        "b create-account alice 1000",
        "b transfer alice bob 200",
        "b balance alice",
    ];
    let mock_stdin = MockStdin::new(commands.into_iter().map(String::from).collect());
    let reader = BufReader::new(mock_stdin);

    let mut blockchain = Blockchain::new(Duration::from_secs(1));
    let mut output = Vec::new();
    blockchain.start_with_mocks(reader, &mut output);

    // Example assertions (adjust based on actual output logic)
    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Account 'alice' created with balance 1000"));
    assert!(output_str.contains("Transferred 200 from 'alice' to 'bob'"));
    assert!(output_str.contains("Balance of 'alice': 800"));
}

}
