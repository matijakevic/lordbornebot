CREATE TABLE IF NOT EXISTS Users (
    ID TEXT PRIMARY KEY NOT NULL UNIQUE, --Twitch ID
    Points INTEGER NOT NULL DEFAULT 100,
    Username TEXT NOT NULL,
    RPGData BLOB
);
