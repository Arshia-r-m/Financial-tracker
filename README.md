# Financial Tracker

**Financial Tracker** is a command-line tool for managing accounts and tracking transactions. Built in Rust, this application uses SQLite for data storage, allowing users to add accounts, record transactions, and view account details. The application provides a simple interface to handle personal finances with basic operations like tracking balances and transaction history.

## Features

- **Add Accounts**: Create accounts with initial balances.
- **Add Transactions**: Record transactions with descriptions, amounts, and dates.
- **Database Management**: Stores data securely using SQLite, automatically creating necessary directories and files.
- **Command-line Interface**: User-friendly CLI powered by `clap` for easy command handling.

## Requirements

Before you begin, ensure you have met the following requirements:
- You have installed [Cargo](https://www.rust-lang.org/tools/install).

## Installation

To install **Financial Tracker**, first clone the repository:

```bash
git clone https://github.com/Arshia-r-m/Financial-tracker.git
cd Financial-tracker
```

Then, install the application locally using Cargo:

```bash
cargo install --path .
```

## Usage

### Adding an Account

To add a new account, use the `add_account` command with the account's name and optional initial balance:

```bash
fin add_account --name "Account Name" --balance 1000
```

- **`--name`**: The name of the account (required).
- **`--balance`**: The starting balance for the account (optional, defaults to 0).

### Adding a Transaction

To record a transaction, use the `add_transaction` command with the account name, amount, date, type, and optional description:

```bash
fin add_transaction --account_name "Account Name" --amount 200 --date "2023-11-01" --type income --description "Grocery shopping"
```

- **`--account_name`**: The name of the account for the transaction (required).
- **`--amount`**: The transaction amount. Use a negative value for withdrawals.
- **`--date`**: The date of the transaction in `YYYY-MM-DD` format (required).
- **`--type`**: Transaction type, possible options are income or expense (required).
- **`--description`**: A description of the transaction (optional).

### Command-line Options

For a list of available commands and options, use:

```bash
fin --help
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