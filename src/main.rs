use blockchain_simulation::Blockchain;
use std::env;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut blockchain = Blockchain::new(Duration::from_secs(10), None);

    if args.len() < 2 || args[1] != "b" {
        print_usage_and_exit();
    }

    match args.get(2).map(String::as_str) {
        Some("start-node") => {
            println!("Starting the B blockchain node...");
            loop {
                blockchain.mine_block();
                std::thread::sleep(Duration::from_secs(10));
            }
        }
        Some("create-account") if args.len() == 5 => {
            let id = &args[3];
            match args[4].parse::<u64>() {
                Ok(starting_balance) => {
                    match blockchain.create_account(id, starting_balance) {
                        Ok(_) => println!("Account '{}' created with balance {}", id, starting_balance),
                        Err(e) => println!("Error creating account: {}", e),
                    }
                }
                Err(_) => println!("Error: Starting balance should be a number"),
            }
        }
        Some("transfer") if args.len() == 6 => {
            let from_account = &args[3];
            let to_account = &args[4];
            match args[5].parse::<u64>() {
                Ok(amount) => {
                    match blockchain.transfer(from_account, to_account, amount) {
                        Ok(_) => println!("Transferred {} from '{}' to '{}'", amount, from_account, to_account),
                        Err(e) => println!("Error transferring funds: {}", e),
                    }
                }
                Err(_) => println!("Error: Amount should be a number"),
            }
        }
        Some("list-accounts") => {
            println!("{}", blockchain.list_accounts());
        }
        Some("balance") if args.len() == 4 => {
            let account = &args[3];
            match blockchain.balance(account) {
                Ok(balance) => println!("Balance of '{}': {}", account, balance),
                Err(e) => println!("Error checking balance: {}", e),
            }
        }
        _ => print_usage_and_exit(),
    }
}

fn print_usage_and_exit() {
    println!("Usage:");
    println!("b start-node");
    println!("b create-account <id-of-account> <starting-balance>");
    println!("b transfer <from-account> <to-account> <amount>");
    println!("b balance <account>");
    println!("b list-accounts");
    std::process::exit(1);
}
