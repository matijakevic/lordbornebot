use bincode::{deserialize, serialize};
use rusqlite::{Connection, Error};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub inventory: Inventory,
    pub state: State,
    pub stats: Stats,
}

const INITIAL_BAG_SLOTS: usize = 5;
const INITIAL_ARMOR_SLOTS: usize = 6;

impl Player {
    pub fn new(stats: Stats) -> Player {
        let mut inventory = Inventory::default();

        for _ in 0..INITIAL_BAG_SLOTS {
            inventory.bag.push(None);
        }

        for _ in 0..INITIAL_BAG_SLOTS {
            inventory.bag.push(None);
        }

        inventory.weapon = Some(InventoryItem {
            name: "Ludwig's Holy Blade".to_string(),
            item: Weapon {
                base_dmg: 1,
                crit_dmg: 4,
                two_handed: false,
                dex_scaling: 2,
                str_scaling: 1,
            },
        });

        Player {
            inventory,
            state: State { hp: stats.vit },
            stats,
        }
    }

    pub fn can_equip_shield(&self) -> bool {
        match &self.inventory.weapon {
            Some(inv_item) => !inv_item.item.two_handed,
            None => true,
        }
    }

    pub fn get_damage(&self) -> (i32, i32) {
        let mut dmg = 0;
        let mut dmg_crit = 0;
        if let Some(inv_item) = &self.inventory.weapon {
            dmg += inv_item.item.base_dmg;
            dmg += self.stats.str * inv_item.item.str_scaling;
            dmg += self.stats.dex * inv_item.item.dex_scaling;

            dmg_crit += inv_item.item.crit_dmg;
        }
        (dmg, dmg_crit)
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub hp: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub dex: i32,
    pub str: i32,
    pub vit: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Weapon {
    pub base_dmg: i32,
    pub crit_dmg: i32,
    pub two_handed: bool,
    pub dex_scaling: i32,
    pub str_scaling: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Armor {
    pub def: i32,
}

#[derive(Serialize, Deserialize)]
pub enum Item {
    Weapon(Weapon),
    Chestplate(Armor),
}

#[derive(Serialize, Deserialize)]
pub struct InventoryItem<T> {
    pub name: String,
    pub item: T,
}

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    // Can hold arbitrary items
    pub weapon: Option<InventoryItem<Weapon>>,

    pub bag: Vec<Option<InventoryItem<Item>>>,
    //pub ring: Option<InventoryItem<Armor>>,
    /*pub necklace: Option<InventoryItem>,
    pub chestplate: Option<InventoryItem>,
    pub helmet: Option<InventoryItem>,
    pub shield: Option<InventoryItem>,*/
}

impl Default for Inventory {
    fn default() -> Inventory {
        Inventory {
            bag: Vec::new(),
            weapon: None,
        }
    }
}

pub fn save_player(connection: &Connection, twitch_id: &str, player: &Player) {
    let data = serialize(player).unwrap();
    connection
        .execute(
            "UPDATE Users SET RPGData=? WHERE ID=?",
            &[&data, &twitch_id],
        )
        .unwrap();
}

pub fn get_twitch_id(connection: &Connection, username: &str) -> Result<String, Error> {
    connection.query_row(
        "SELECT ID FROM Users WHERE Username=?",
        &[&username.to_lowercase()],
        |row| row.get(0),
    )
}

pub fn load_player(connection: &Connection, twitch_id: &str) -> Result<Option<Player>, Error> {
    let data: Option<Vec<u8>> = connection.query_row(
        "SELECT RPGData FROM Users WHERE ID=?",
        &[&twitch_id],
        |row| row.get(0),
    )?;

    match data {
        Some(player_data) => Ok(Some(deserialize(&player_data).unwrap())),
        None => Ok(None),
    }
}

pub fn delete_player(connection: &Connection, twitch_id: &str) -> Result<i32, Error> {
    connection.execute("UPDATE Users SET RPGData=NULL WHERE ID=?", &[&twitch_id])
}
