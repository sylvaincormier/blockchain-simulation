
use blockchain_simulation::Blockchain;
use std::env;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Initialize the blockchain instance outside of the command check
    let mut blockchain = Blockchain::new(Duration::from_secs(10), None);


    // The first argument should be the program name, and the second should be 'b'
    if args.len() < 2 || args[1] != "b" {
        println!("Usage:");
        println!("b start-node");
        println!("b create-account <id-of-account> <starting-balance>");
        println!("b transfer <from-account> <to-account> <amount>");
        println!("b balance <account>");
        println!("b list-accounts");
        return;
    }

    // Start processing from the third argument as the command
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
            let starting_balance: u64 = args[4]
                .parse()
                .expect("Starting balance should be a number");
            blockchain.create_account(id, starting_balance).unwrap();
            println!("Account '{}' created with balance {}", id, starting_balance);
        }
        Some("transfer") if args.len() == 6 => {
            let from_account = &args[3];
            let to_account = &args[4];
            let amount: u64 = args[5].parse().expect("Amount should be a number");
            blockchain
                .transfer(from_account, to_account, amount)
                .unwrap();
            println!(
                "Transferred {} from '{}' to '{}'",
                amount, from_account, to_account
            );
        }
        Some("list-accounts") => {
            let accounts_info = blockchain.list_accounts();
            println!("{}", accounts_info);
        }
        Some("balance") if args.len() == 4 => {
            let account = &args[3];
            let balance = blockchain.balance(account).unwrap();
            println!("Balance of '{}': {}", account, balance);
        }
        _ => {
            println!("Invalid command or wrong number of arguments");
            println!("Usage:");
            println!("b start-node");
            println!("b create-account <id-of-account> <starting-balance>");
            println!("b transfer <from-account> <to-account> <amount>");
            println!("b balance <account>");
            println!("b list-accounts");
        }
    }
}
