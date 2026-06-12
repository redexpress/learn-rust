-- sqlite

CREATE TABLE users (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    username    TEXT NOT NULL UNIQUE,
    email       TEXT NOT NULL UNIQUE,
    created_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE orders (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id      INTEGER NOT NULL,
    total_price  REAL NOT NULL DEFAULT 0,
    status       TEXT NOT NULL,
    created_at   DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
FOREIGN KEY (user_id)
REFERENCES users(id)
);

CREATE TABLE order_items (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id      INTEGER NOT NULL,
    product_name  TEXT NOT NULL,
    quantity      INTEGER NOT NULL,
    unit_price    REAL NOT NULL,
    image         BLOB,
FOREIGN KEY (order_id)
REFERENCES orders(id)
);

CREATE TABLE demo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    price REAL,
    create_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    image BLOB
);