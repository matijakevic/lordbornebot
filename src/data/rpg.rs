use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub inventory: Inventory,
    pub state: State,
    pub stats: Stats,
}

impl Player {
    pub fn new(stats: Stats) -> Player {
        Player {
            inventory: Inventory {
                bag: Vec::new(),
                weapon: InventorySlot {
                    item: None,
                    slot_type: InventorySlotType::Weapon,
                },
                armor: Vec::new(),
            },
            state: State { hp: stats.vit },
            stats,
        }
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
pub enum InventorySlotType {
    Any,
    Helmet,
    Ring,
    Necklace,
    Leggings,
    Chestplate,
    Boots,
    Weapon,
}

#[derive(Serialize, Deserialize)]
pub struct WeaponItem {
    pub base_dmg: i32,
    pub crit_dmg: i32,
    pub two_handed: bool,
    pub dex_scaling: i32,
    pub str_scaling: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ArmorItem {
    pub def: i32,
    pub slot: InventorySlotType,
}

#[derive(Serialize, Deserialize)]
pub enum Item {
    Weapon(String, WeaponItem),
    Armor(String, ArmorItem),
}

#[derive(Serialize, Deserialize)]
pub struct InventorySlot {
    pub item: Option<Item>,
    pub slot_type: InventorySlotType,
}

#[derive(Serialize, Deserialize)]
pub struct Inventory {
    pub bag: Vec<InventorySlot>,
    pub weapon: InventorySlot,
    pub armor: Vec<InventorySlot>,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub players: HashMap<String, Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: HashMap::new(),
        }
    }
}
