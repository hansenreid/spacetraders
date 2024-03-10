 CREATE TABLE IF NOT EXISTS agents (
    account_id TEXT,
    symbol TEXT PRIMARY KEY NOT NULL,
    headquarters TEXT NOT NULL,
    credits INTEGER NOT NULL,
    starting_faction TEXT NOT NULL,
    ship_count INTEGER NOT NULL,
    token TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);
