CREATE TABLE IF NOT EXISTS my_data (
    id integer PRIMARY KEY AUTOINCREMENT NOT NULL,
    name text NOT NULL UNIQUE,
    created_time datetime NOT NULL DEFAULT (STRFTIME ('%Y-%m-%d %H:%M:%f', 'NOW'))
);

