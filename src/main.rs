use blockchain_simulation::Blockchain;
use std::env;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage_and_exit();
    }

    match args[1].as_str() {
        "b" => {
            match args.get(2).map(String::as_str) {
                Some("start-node") => {
                    let blockchain = Arc::new(Mutex::new(Blockchain::new(Duration::from_secs(10))));
                    println!("Starting blockchain node...");
                    Blockchain::start_node(blockchain);
                    // Keep the main thread alive to allow mining in the background
                    loop {
                        std::thread::sleep(Duration::from_secs(60));
                    }
                }
                Some("create-account") if args.len() == 5 => {
                    let command = format!("create-account {} {}", args[3], args[4]);
                    send_command_to_node(command);
                }
                Some("transfer") if args.len() == 6 => {
                    let command = format!("transfer {} {} {}", args[3], args[4], args[5]);
                    send_command_to_node(command);
                }
                Some("list-accounts") => {
                    let command = "list-accounts".to_string();
                    send_command_to_node(command);
                }
                Some("balance") if args.len() == 4 => {
                    let command = format!("balance {}", args[3]);
                    send_command_to_node(command);
                }
                _ => print_usage_and_exit(),
            }
        }
        _ => print_usage_and_exit(),
    }
}

fn print_usage_and_exit() {
    println!("Usage:");
    println!("b start-node");
    println!("b create-account <id> <balance>");
    println!("b transfer <from-account> <to-account> <amount>");
    println!("b list-accounts");
    println!("b balance <account>");
    std::process::exit(1);
}

fn send_command_to_node(command: String) {
    let command = command.trim_end(); // Ensure no trailing newline or spaces
    println!("Sending command: '{}'", command);
    match TcpStream::connect("127.0.0.1:3000") {
        Ok(mut stream) => {
            if let Err(e) = stream.write_all(command.as_bytes()) {
                println!("Failed to send command: {}", e);
                return;
            }
            stream.flush().expect("Failed to flush the stream");
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
            return;
        }
    }
}
