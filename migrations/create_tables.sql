CREATE TABLE IF NOT EXISTS `Users` (
  TwitchID TEXT PRIMARY KEY NOT NULL UNIQUE,
  Points   INTEGER          NOT NULL DEFAULT 100,
  Username TEXT             NOT NULL
);

CREATE TABLE IF NOT EXISTS `Weapons` (
  ID              INTEGER PRIMARY KEY  AUTOINCREMENT NOT NULL UNIQUE,
  Name            TEXT                               NOT NULL UNIQUE,
  BaseDamage      INTEGER                            NOT NULL,
  MagicDamage     INTEGER                            NOT NULL,
  Critical        INTEGER                            NOT NULL,
  InventoryItemID INTEGER UNIQUE,
  TwoHanded       BOOL                               NOT NULL,
  FOREIGN KEY (InventoryItemID) REFERENCES InventoryItem (ID)
);

CREATE TABLE IF NOT EXISTS `ArmorSlots` (
  ID   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
  Name TEXT UNIQUE                       NOT NULL
);

CREATE TABLE IF NOT EXISTS `Armor` (
  ID              INTEGER PRIMARY KEY  AUTOINCREMENT NOT NULL UNIQUE,
  Name            TEXT                               NOT NULL UNIQUE,
  Defense         INTEGER                            NOT NULL,
  MagicDefense    INTEGER                            NOT NULL,
  Slot            INTEGER                            NOT NULL,
  InventoryItemID INTEGER UNIQUE,
  FOREIGN KEY (Slot) REFERENCES ArmorSlots (ID),
  FOREIGN KEY (InventoryItemID) REFERENCES InventoryItem (ID)
);

CREATE TABLE IF NOT EXISTS `RPGUsers` (
  ID                           TEXT PRIMARY KEY NOT NULL,
  HP                           INTEGER          NOT NULL,
  Strength                     INTEGER          NOT NULL,
  Dexterity                    INTEGER          NOT NULL,
  WeaponRPGUserInventoryItemID INTEGER,

  FOREIGN KEY (ID) REFERENCES Users (TwitchID)
    ON DELETE CASCADE,
  FOREIGN KEY (WeaponRPGUserInventoryItemID) REFERENCES RPGUserInventoryItems (ID)
    ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS `RPGUserArmor` (
  RPGUserID                   TEXT    NOT NULL,
  ArmorRPGUserInventoryItemID INTEGER NOT NULL,
  FOREIGN KEY (RPGUserID) REFERENCES RPGUsers (ID)
    ON DELETE CASCADE,
  FOREIGN KEY (ArmorRPGUserInventoryItemID) REFERENCES RPGUserInventoryItems (ID)
    ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `InventoryItem` (
  ID   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  Name TEXT                              NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS `RPGUserInventoryItems` (
  ID              INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  RPGUserID       TEXT                              NOT NULL,
  InventoryItemID INTEGER                           NOT NULL,
  Amount          INTEGER                           NOT NULL,
  FOREIGN KEY (RPGUserID) REFERENCES RPGUsers (ID)
    ON DELETE CASCADE,
  FOREIGN KEY (InventoryItemID) REFERENCES InventoryItem (ID)
    ON DELETE CASCADE
);