# Blockchain Simulation 'B' 🚀

This project is a simple blockchain simulation named 'B'. It demonstrates basic blockchain operations like creating accounts, transferring funds, and viewing account balances. The blockchain creates new blocks at regular intervals, simulating the confirmation of transactions.

## 📋 Getting Started

### Prerequisites 📚

- Rust programming language environment [Rust Installation](https://www.rust-lang.org/tools/install)
- Cargo, Rust's package manager (comes with Rust installation)

### 🚴 Running the Simulation

1. **Start the Blockchain Node** 🌟

   This starts a local B blockchain server that mines blocks at regular intervals (every 10 seconds).

   ```
   cargo run -- b start-node
```
2. **Keep this running in a separate terminal window or tab.**

    3. Create an Account 🏦

    Creates a new account with a specified starting balance.

    '''

cargo run -- b create-account <id-of-account> <starting-balance>
'''
Replace <id-of-account> with the desired account identifier and <starting-balance> with the initial balance.

3. Transfer Funds 💸

4. Transfers funds from one account to another.

```

cargo run -- b transfer <from-account> <to-account> <amount>
```
Replace <from-account> and <to-account> with the respective account identifiers and <amount> with the number of funds to transfer.

5. Check Account Balance 💼

Displays the balance of the specified account.

```

    cargo run -- b balance <account>
```
    Replace <account> with the account identifier whose balance you want to check.

6. 🧪 Running Tests

To run the tests, execute the following command:

```

cargo test
```

This command runs all tests in the project, ensuring that the blockchain functionality is working as expected.

## 🏗 Architecture

    main.rs: Entry point of the application that handles the command-line interface.
    lib.rs: Contains the core blockchain logic including account creation, fund transfer, block mining, and balance checking.

## Miscellaneous 🌈

    The simulation is single-threaded and does not include networking or cryptography features.
    Data is not persisted; restarting the simulation will reset the blockchain.

