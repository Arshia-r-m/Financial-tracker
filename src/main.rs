use dirs::home_dir;
use rusqlite::{params, Connection, Result};
use std::fs;
use clap::{Arg, Command};

#[derive(Debug)]
struct Account {
    id: Option<i32>,
    name: String,
    balance: Option<u32>,
}

#[derive(Debug)]
struct Transaction {
    id: Option<i32>,
    account_name: String,
    amount: i32,
    date: String,
    description: Option<String>,
}

fn main() {
    let matches = Command::new("Finance Tracker")
        .version("1.0")
        .author("Arshia")
        .about("Tracks accounts and transactions")
        .subcommand(
            Command::new("add-account")
                .about("Add a new account")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .value_name("NAME")
                        .help("The name of the account")
                        .required(true),
                )
                .arg(
                    Arg::new("balance")
                        .short('b')
                        .long("balance")
                        .value_name("BALANCE")
                        .help("Initial balance of the account"),
                ),
        )
        .subcommand(Command::new("list-accounts").about("List all accounts"))
        .subcommand(
            Command::new( "remove-account")
            .about("Remove an account")
            .arg(
                Arg::new("id")
                .short('i')
                .long("id")
                .value_name("ID")
                .help("The id of the account to remove")
                .required(true),
            ),
            )
        .subcommand(
            Command::new("add-transaction")
                .about("Add a new transaction")
                .arg(
                    Arg::new("account-name")
                        .short('a')
                        .long("account-name")
                        .value_name("ACCOUNT_NAME")
                        .help("The name of the account for the transaction")
                        .required(true),
                )
                .arg(
                    Arg::new("amount")
                        .short('m')
                        .long("amount")
                        .value_name("AMOUNT")
                        .help("The transaction amount")
                        .required(true),
                )
                .arg(
                    Arg::new("date")
                        .short('d')
                        .long("date")
                        .value_name("DATE")
                        .help("The date of the transaction")
                        .required(true),
                )
                .arg(
                    Arg::new("description")
                        .short('s')
                        .long("description")
                        .value_name("DESCRIPTION")
                        .help("A description of the transaction"),
                )
                .arg(
                    Arg::new("type")
                        .short('t')
                        .long("type")
                        .value_name("TYPE")
                        .help("The type of the transaction")
                        .value_parser(["income", "expense"])
                        .required(true),
                ),
        )
        .subcommand(Command::new("list-expenses").about("List all expenses"))
        .subcommand(
            Command::new("remove-expense")
            .about("Remove an expense")
            .arg(
                Arg::new("id")
                .short('i')
                .long("id")
                .value_name("ID")
                .help("The id of the expense to remove")
                .required(true)
            )
        )
        .subcommand(Command::new("list-incomes").about("List all incomes"))
        .subcommand(
            Command::new("remove-income")
            .about("Remove an income")
            .arg(
                Arg::new("id")
                .short('i')
                .long("id")
                .value_name("ID")
                .help("The id of the expense to remove")
                .required(true)
            )
        )
        .get_matches();

    let conn = database_connection().expect("Failed to connect to database");
    
    match matches.subcommand() {
        Some(("add-account", sub_m)) => {
            let name = sub_m.get_one::<String>("name").expect("name is required");
            let balance = sub_m.get_one::<String>("balance").unwrap_or(&"0".to_string()).parse::<u32>().unwrap();

            let account = Account {
                id: None,
                name: name.clone(),
                balance: Some(balance),
            };
            let _ = add_account(&conn, account);
        }
        Some(("list-accounts", _)) => {
            let _ = list_accounts(&conn);
        }
        Some(("remove-account", sub_m)) => {
            let id = sub_m.get_one::<String>("id").unwrap().parse::<i32>().unwrap();
            let _ = remove_account(&conn, id);
        }
        Some(("add-transaction", sub_m)) => {
            let account_name = sub_m.get_one::<String>("account-name").expect("account-name is required");
            let amount = sub_m.get_one::<String>("amount").expect("amount is required").parse::<i32>().unwrap();
            let date = sub_m.get_one::<String>("date").expect("date is required").to_string();
            let description = sub_m.get_one::<String>("description").map(|s| s.to_string());

            let transaction = Transaction {
                id: None,
                account_name: account_name.clone(),
                amount,
                date,
                description,
            };
            let transaction_type = sub_m.get_one::<String>("type").expect("type is required");
            if transaction_type == "income" {
                let _ = add_income(&conn, transaction);
            } else if transaction_type == "expense" {
                let _ = add_expense(&conn, transaction);
            } else {
                println!("Invalid transaction type");
            }
        }
        Some(("list-expenses", _)) => {
            let _ = list_expenses(&conn);
        }
        Some(("remove-expense", sub_m)) => {
            let id = sub_m.get_one::<String>("id").unwrap().parse::<i32>().unwrap();
            let _ = remove_expense(&conn, id);
        }
        Some(("list-incomes", _)) => {
            let _ = list_incomes(&conn);
        }
        Some(("remove-income", sub_m)) => {
            let id = sub_m.get_one::<String>("id").unwrap().parse::<i32>().unwrap();
            let _ = remove_income(&conn, &id);
        }
        _ => println!("Use --help for more information."),
    }
}

fn database_connection() -> Result<Connection> {
    let mut db_path = home_dir().unwrap();
    db_path.push(".ft/ft_database.db");
    if let Some(parent) = db_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name  TEXT NOT NULL UNIQUE,
        balance INTEGER DEFAULT 0
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS expenses (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        account_name TEXT NOT NULL,
        amount INTEGER NOT NULL,
        date TEXT NOT NULL,
        description TEXT,
        FOREIGN KEY (account_name) REFERENCES accounts (name) ON DELETE CASCADE
        );",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS incomes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        account_name TEXT NOT NULL,
        amount INTEGER NOT NULL,
        date TEXT NOT NULL,
        description TEXT,
        FOREIGN KEY (account_name) REFERENCES accounts (name) ON DELETE CASCADE
        );",
        [],
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_balance_after_expense
        AFTER INSERT ON expenses
        FOR EACH ROW
        BEGIN
            UPDATE accounts
            SET balance = balance - NEW.amount
            WHERE name = NEW.account_name;
        END;",
        [],
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_balance_after_income
        AFTER INSERT ON incomes
        FOR EACH ROW
        BEGIN
            UPDATE accounts
            SET balance = balance + NEW.amount
            WHERE name = NEW.account_name;
        END;",
        [],
    )?;

    conn.execute(
"CREATE TRIGGER IF NOT EXISTS update_balance_after_expense_deletation
        AFTER DELETE ON expenses
        FOR EACH ROW
        BEGIN
            UPDATE accounts
            SET balance = balance + OLD.amount
            WHERE name = OLD.account_name;
        END;",
         [])?;
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS update_balance_after_income_deletation
        AFTER DELETE ON incomes
        FOR EACH ROW
        BEGIN
            UPDATE accounts
            SET balance = balance - OLD.amount
            WHERE name = OLD.account_name;
        END;",
         [])?;
    Ok(conn)
}

//Accounts functions

fn list_accounts(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, balance FROM accounts")?;
    let account_iter = stmt.query_map([], |row| {
        Ok(Account {
            id: row.get(0)?,
            name: row.get(1)?,
            balance: row.get(2)?,
        })
    })?;
    for account in account_iter {
        match account {
            Ok(_) => {
                let account = account.unwrap();
                println!(
                    "Account id: {:?}, Account name {:?}, Balance: {:?}",
                    &account.id.unwrap(),
                    &account.name,
                    &account.balance.unwrap()
                );
            }
            Err(e) => println!("Failed to retrieve account: {:?}", e),
        }
    }
    Ok(())
}

fn add_account(conn: &Connection, account: Account) -> Result<()> {
    let result = conn.execute(
        "INSERT INTO accounts (name, balance) VALUES (?1, ?2)",
        params![account.name, account.balance],
    );
    match result {
        Ok(_) => println!("Account added successfully."),
        Err(e) => println!("Failed to add account: {:?}", e),
    };
    Ok(())
}

fn remove_account(conn: &Connection, account: i32) -> Result<()> {
    println!("Removing account: {}", &account);
    let result = conn.execute("DELETE FROM accounts WHERE id = ?1", params![account]);
    match result {
        Ok(_) => println!("Account removed successfully."),
        Err(e) => println!("Failed to remove account: {:?}", e),
    };
    Ok(())
}

//Expenses functions

fn list_expenses(conn: &Connection) -> Result<()> {
    let mut stmt =
        conn.prepare("SELECT id, account_name, amount, date, description FROM expenses")?;
    let expense_iter = stmt.query_map([], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            account_name: row.get(1)?,
            amount: row.get(2)?,
            date: row.get(3)?,
            description: row.get(4)?,
        })
    })?;
    for expense in expense_iter {
        match expense {
            Ok(_) => {
                let expense = expense.unwrap();
                println!(
                    "Expense id: {:?}, Account name: {:?}, Amount: {:?}, Date: {:?}, Description: {:?}",
                    &expense.id.unwrap(),
                    &expense.account_name,
                    &expense.amount,
                    &expense.date,
                    &expense.description
                );
            }
            Err(e) => println!("Failed to retrieve expense: {:?}", e),
        }
    }
    Ok(())
}

fn add_expense(conn: &Connection, expense: Transaction) -> Result<()> {
    println!("Adding expense: {:?}", expense);
    let result = conn.execute(
        "INSERT INTO expenses (account_name, amount, date, description) VALUES (?1, ?2, ?3, ?4)",
        params![
            expense.account_name,
            expense.amount,
            expense.date,
            expense.description
        ],
    );
    match result {
        Ok(_) => println!("Expense added successfully."),
        Err(e) => println!("Failed to add expense: {:?}", e),
    };
    Ok(())
}

fn remove_expense(conn: &Connection, expense: i32) -> Result<()> {
    println!("Removing expense: {}", &expense);
    let result = conn.execute("DELETE FROM expenses WHERE id = ?1", params![expense]);
    match result {
        Ok(_) => println!("Expense removed successfully."),
        Err(e) => println!("Failed to remove expense: {:?}", e),
    };
    Ok(())
}

//Incomes functions

fn list_incomes(conn: &Connection) -> Result<()> {
    let mut stmt =
        conn.prepare("SELECT id, account_name, amount, date, description FROM incomes")?;
    let income_iter = stmt.query_map([], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            account_name: row.get(1)?,
            amount: row.get(2)?,
            date: row.get(3)?,
            description: row.get(4)?,
        })
    })?;
    for income in income_iter {
        match income {
            Ok(_) => {
                let income = income.unwrap();
                println!(
                    "Income id: {:?}, Account name: {:?}, Amount: {:?}, Date: {:?}, Description: {:?}",
                    &income.id.unwrap(),
                    &income.account_name,
                    &income.amount,
                    &income.date,
                    &income.description
                );
            }
            Err(e) => println!("Failed to retrieve income: {:?}", e),
        }
    }
    Ok(())
}

fn add_income(conn: &Connection, income: Transaction) -> Result<()> {
    println!("Adding income: {:?}", income);
    let result = conn.execute(
        "INSERT INTO incomes (account_name, amount, date, description) VALUES (?1, ?2, ?3, ?4)",
        params![
            income.account_name,
            income.amount,
            income.date,
            income.description
        ],
    );
    match result {
        Ok(_) => println!("Income added successfully."),
        Err(e) => println!("Failed to add income: {:?}", e),
    };
    Ok(())
}   

fn remove_income(conn: &Connection, income: &i32) -> Result<()> {
    println!("Removing income: {}", income);
    let result = conn.execute("DELETE FROM incomes WHERE id = ?1", params![income]);
    match result {
        Ok(_) => println!("Income removed successfully."),
        Err(e) => println!("Failed to remove income: {:?}", e),
    };
    Ok(())
}