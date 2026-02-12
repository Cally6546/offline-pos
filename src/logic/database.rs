use rusqlite::{Connection, Result};

pub fn initialize_db() -> Result<Connection> {
    // This creates a file named "pos_data.db" in your folder
    let conn = Connection::open("pos_data.db")?;

    // Create a table for products if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            price   REAL NOT NULL
        )",
        [],
    )?;

    println!("Database initialized successfully!");
    Ok(conn)
}
