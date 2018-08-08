use rusqlite::{Connection, Error};

pub struct Stats {
    pub vitality: i32,
    pub strength: i32,
    pub dexterity: i32,
}

pub struct State {
    pub hp: i32,
}

pub enum ItemType {
    Item(String),
    Weapon,
    Consumable,
    Armor,
}

pub struct InventoryItem {
    pub item: ItemType,
    pub amount: i32,
}

pub fn create_player(
    connection: &Connection,
    twitch_id: &str,
    stats: (i32, i32, i32),
) -> Result<(), Error> {
    connection.execute("INSERT INTO Players (UserID) VALUES (?)", &[&twitch_id])?;
    connection.execute(
        "INSERT INTO Stats (PlayerID, Vitality, Strength, Dexterity) VALUES (?,?,?,?)",
        &[&twitch_id, &stats.0, &stats.1, &stats.2],
    )?;
    connection.execute(
        "INSERT INTO State (PlayerID, HP) VALUES (?,?)",
        &[&twitch_id, &stats.0],
    )?;
    Ok(())
}

pub fn get_player_info(connection: &Connection, username: &str) -> Result<(Stats, State), Error> {
    info!("Getting player info for {}", username);

    let data = connection.query_row(
        "SELECT Stats.Vitality, Stats.Strength, Stats.Dexterity, State.HP FROM Stats WHERE PlayerID=(SELECT ID FROM Users WHERE Username=?)",
        &[&username],
        |row| (Stats {
            vitality: row.get(0),
            strength: row.get(1),
            dexterity: row.get(2),
        }, State {hp: row.get(3)}),
    )?;

    return Ok(data);
}

pub fn get_item(connection: &Connection, id: i32) -> Result<ItemType, Error> {
    connection.query_row("SELECT Name FROM Items WHERE ID=?", &[&id], |row| {
        ItemType::Item(row.get(0))
    })
}

pub fn get_all_player_inventory(
    connection: &Connection,
    twitch_id: &str,
) -> Result<Vec<InventoryItem>, Error> {
    let mut stmt =
        connection.prepare("SELECT (ItemID, Amount) FROM InventoryItem WHERE PlayerID=?")?;
    let mut rows = stmt.query(&[&twitch_id])?;
    let mut items = Vec::new();

    while let Some(row) = rows.next() {
        let row = row?;
        let item = get_item(connection, row.get(0))?;
        let amount = row.get(1);
        items.push(InventoryItem { item, amount });
    }

    Ok(items)
}

pub fn set_player_weapon() {}
