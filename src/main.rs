use blockchain_simulation::Blockchain;
use std::env;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut blockchain = Blockchain::new(Duration::from_secs(10));

    match args.get(1).map(String::as_str) {
        Some("start-node") => {
            println!("Starting the B blockchain node...");
            loop {
                blockchain.mine_block();
                std::thread::sleep(Duration::from_secs(10));
            }
        },
        Some("create-account") if args.len() == 4 => {
            let id = &args[2];
            let starting_balance: u64 = args[3].parse().expect("Starting balance should be a number");
            blockchain.create_account(id, starting_balance).unwrap();
            println!("Account '{}' created with balance {}", id, starting_balance);
        },
        Some("transfer") if args.len() == 5 => {
            let from_account = &args[2];
            let to_account = &args[3];
            let amount: u64 = args[4].parse().expect("Amount should be a number");
            blockchain.transfer(from_account, to_account, amount).unwrap();
            println!("Transferred {} from '{}' to '{}'", amount, from_account, to_account);
        },
        Some("balance") if args.len() == 3 => {
            let account = &args[2];
            let balance = blockchain.balance(account).unwrap();
            println!("Balance of '{}': {}", account, balance);
        },
        _ => {
            println!("Invalid command or wrong number of arguments");
            println!("Usage:");
            println!("start-node");
            println!("create-account <id-of-account> <starting-balance>");
            println!("transfer <from-account> <to-account> <amount>");
            println!("balance <account>");
        },
    }
}
