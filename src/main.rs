use dirs::home_dir;
use rusqlite::{params, Connection, Result};
use std::fs;

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
    amount: u32,
    date: String,
    description: Option<String>,
}

fn main() {
    let conn = database_connection().expect("Failed to connect to database");

    let account = Account {
        id: Some(1),
        name: "Bank".to_string(),
        balance: Some(100),
    };
    let transaction = Transaction {
        id: Some(1),
        account_name: "Bank".to_string(),
        amount: 10,
        date: "2021-01-01".to_string(),
        description: Some("Bought a book".to_string()),
    };
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

fn remove_account(conn: &Connection, account: Account) -> Result<()> {
    println!("Removing account: {:?}", account.id.unwrap());
    let result = conn.execute("DELETE FROM accounts WHERE id = ?1", params![1]);
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

fn remove_expense(conn: &Connection, expense: Transaction) -> Result<()> {
    println!("Removing expense: {:?}", expense.id.unwrap());
    let result = conn.execute("DELETE FROM expenses WHERE id = ?1", params![1]);
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

fn remove_income(conn: &Connection, income: Transaction) -> Result<()> {
    println!("Removing income: {:?}", income.id.unwrap());
    let result = conn.execute("DELETE FROM incomes WHERE id = ?1", params![1]);
    match result {
        Ok(_) => println!("Income removed successfully."),
        Err(e) => println!("Failed to remove income: {:?}", e),
    };
    Ok(())
}