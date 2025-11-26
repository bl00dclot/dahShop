CREATE TABLE orders (
    id INTEGER PRIMARY KEY NOT NULL,
    order_id TEXT UNIQUE NOT NULL,
    zk_proof_hash TEXT NOT NULL,
    payment_tx_hash TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at BIGINT NOT NULL
);