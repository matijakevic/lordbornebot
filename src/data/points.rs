use rusqlite::{Connection, Error};

pub fn get_points_by_username(connection: &Connection, username: &str) -> Result<i32, Error> {
    connection.query_row(
        "SELECT Points FROM Users WHERE Username=? LIMIT 1",
        &[&username],
        |row| Ok(row.get(0)),
    )?
}

pub fn get_points(connection: &Connection, id: &str) -> Result<i32, Error> {
    connection.query_row(
        "SELECT Points FROM Users WHERE ID=? LIMIT 1",
        &[&id],
        |row| Ok(row.get(0)),
    )?
}

pub fn set_points(connection: &Connection, id: &str, points: i32) -> Result<i32, Error> {
    connection.execute("UPDATE Users SET Points=? WHERE ID=?", &[&points, &id])
}
