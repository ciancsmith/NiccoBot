CREATE TABLE IF NOT EXISTS game_ids (
                                     id INTEGER PRIMARY KEY AUTOINCREMENT,
                                     discord_username TEXT UNIQUE NOT NULL,
                                     platforms JSON DEFAULT '{}');