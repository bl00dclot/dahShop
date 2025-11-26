CREATE TABLE products (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    price_xmr TEXT NOT NULL,
    description TEXT NOT NULL,
    image_ipfs_hash TEXT NOT NULL,
    in_stock BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);