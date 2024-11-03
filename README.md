# Financial Tracker

**Financial Tracker** is a command-line tool for managing accounts and tracking transactions. Built in Rust, this application uses SQLite for data storage, allowing users to add accounts, record transactions, and view account details. The application provides a simple interface to handle personal finances with basic operations like tracking balances and transaction history.

## Features

- **Add Accounts**: Create accounts with initial balances.
- **Add Transactions**: Record transactions with descriptions, amounts, and dates.
- **Database Management**: Stores data securely using SQLite, automatically creating necessary directories and files.
- **Command-line Interface**: Interact with the application using intuitive CLI commands powered by `clap`.

## Requirements

- **Rust**: Ensure you have Rust installed. [Install Rust](https://www.rust-lang.org/tools/install)
- **SQLite**: Required for database functionality. Rust's `rusqlite` crate handles SQLite interactions.

## Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/Arshia-r-m/Financial-tracker.git
   cd Financial-tracker
   ```

2. **Build the Project**:

   ```bash
   cargo build --release
   ```

3. **Run the Application**:

   ```bash
   cargo run -- <command>
   ```

## Usage

### Adding an Account

To add a new account, use the `add_account` command with the account's name and optional initial balance:

```bash
cargo run -- add_account --name "Account Name" --balance 1000
```

- **`--name`**: The name of the account (required).
- **`--balance`**: The starting balance for the account (optional, defaults to 0).

### Adding a Transaction

To record a transaction, use the `add_transaction` command with the account name, amount, date, and optional description:

```bash
cargo run -- add_transaction --account_name "Account Name" --amount -200 --date "2023-11-01" --description "Grocery shopping"
```

- **`--account_name`**: The name of the account for the transaction (required).
- **`--amount`**: The amount for the transaction. Use a negative value for withdrawals.
- **`--date`**: The date of the transaction in `YYYY-MM-DD` format (required).
- **`--description`**: A description of the transaction (optional).

### Command-line Options

- Use `--help` for a list of available commands and options:

  ```bash
  cargo run -- --help
  ```

## Project Structure

- **`src/main.rs`**: Main file containing CLI handlers and application logic.
- **`src/`**: Contains helper modules for database connections and transaction handling.
- **`.ft/ft_database.db`**: SQLite database file created in your home directory to store account and transaction data.

## Contributing

Contributions are welcome! If you'd like to contribute:

1. Fork the repository.
2. Create a new branch for your feature.
3. Submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Contact

For any questions or feedback, please contact [Arshia R. M.](https://github.com/Arshia-r-m).
