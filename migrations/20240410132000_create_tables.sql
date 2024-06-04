CREATE TABLE IF NOT EXISTS game_ids (
                                     id INTEGER PRIMARY KEY AUTOINCREMENT,
                                     discord_username TEXT UNIQUE NOT NULL,
                                     platforms JSON DEFAULT '{}');
CREATE TABLE smurfs (
                        id INTEGER PRIMARY KEY AUTOINCREMENT ,
                        account_name TEXT NOT NULL,
                        salt TEXT NOT NULL,
                        nonce TEXT NOT NULL,
                        password TEXT NOT NULL,
                        platform TEXT NOT NULL,
                        info TEXT
);

CREATE TABLE auth_keys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_name TEXT NOT NULL UNIQUE,
    auth_key TEXT NOT NULL UNIQUE,
    salt TEXT NOT NULL
);