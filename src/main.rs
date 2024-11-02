use dirs::home_dir;
use rusqlite::{params, Connection, Result};
use std::{env, fs};
fn main() {
    let conn = match database_connection() {
        Ok(conn) => conn,
        Err(_e) => {
            println!("{:?}", _e);
            panic!()
        }
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
        id    INTEGER PRIMARY KEY AUTOINCREMENT,
        name  TEXT NOT NULL UNIQUE,
        balance INTEGER DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS expenses (
        account_name TEXT NOT NULL,
        amount INTEGER NOT NULL,
        date TEXT NOT NULL,
        description TEXT,
        FOREIGN KEY (account_name) REFERENCES accounts (name)
        );

        CREATE TABLE IF NOT EXISTS incomes (
        account_name TEXT NOT NULL,
        amount INTEGER NOT NULL,
        date TEXT NOT NULL,
        description TEXT,
        FOREIGN KEY (account_name) REFERENCES accounts (name)
        );

        CREATE TRIGGER update_balance_after_expense IF NOT EXISTS
        AFTER INSERT ON expenses
        FOR EACH ROW
        BEGIN
            UPDATE accounts
            SET balance = balance - NEW.amount
            WHERE name = NEW.account_name;
        END;

        CREATE TRIGGER update_balance_after_expense IF NOT EXISTS
        AFTER INSERT ON incomes
        FOR EACH ROW
        BEGIN
            UPDATE accounts
            SET balance = balance + NEW.amount
            WHERE name = NEW.account_name;
        END;
        ",
        [],
    )?;
    Ok(conn)
}
