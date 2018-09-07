use chrono::{DateTime, Utc};
use rusqlite::{Connection, Error};

pub struct AFKStatus {
    pub afk: bool,
    pub reason: String,
    pub time: DateTime<Utc>,
}

pub fn get_afk_status(connection: &Connection, user_id: &str) -> Result<AFKStatus, Error> {
    connection.query_row(
        "SELECT AFK, Reason, Time FROM AFKStatus WHERE ID=? LIMIT 1",
        &[&user_id],
        |row| AFKStatus {
            afk: row.get(0),
            reason: row.get(1),
            time: row.get(2),
        },
    )
}

pub fn get_afk_status_by_username(
    connection: &Connection,
    username: &str,
) -> Result<AFKStatus, Error> {
    connection.query_row(
        "SELECT AFK, Reason, Time FROM AFKStatus WHERE ID=(SELECT ID FROM Users WHERE Username=? LIMIT 1) LIMIT 1",
        &[&username.to_lowercase()],
        |row| AFKStatus {
            afk: row.get(0),
            reason: row.get(1),
            time: row.get(2),
        },
    )
}

pub fn set_afk_status(connection: &Connection, user_id: &str, reason: &str) -> Result<(), Error> {
    connection.execute(
        "UPDATE AFKStatus SET AFK=1, REASON=?, TIME=DATETIME('now') WHERE ID=?",
        &[&reason, &user_id],
    )?;
    Ok(())
}

pub fn unset_afk_status(connection: &Connection, user_id: &str) -> Result<(), Error> {
    connection.execute(
        "UPDATE AFKStatus SET AFK=0, REASON='' WHERE ID=?",
        &[&user_id],
    )?;
    Ok(())
}
