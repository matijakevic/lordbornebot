use rusqlite::{Connection, Error};

/// Checks whether user row exists in the database. Creates one if it doesn't exist.
pub fn check_user(connection: &Connection, user_id: &str, username: &str) -> Result<i32, Error> {
    connection.execute(
        "INSERT OR IGNORE INTO Users (ID, Username) VALUES (?, ?)",
        &[&user_id, &username.to_lowercase()],
    )
}
