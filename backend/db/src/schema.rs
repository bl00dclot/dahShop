// @generated automatically by Diesel CLI.

diesel::table! {
    orders (id) {
        id -> Integer,
        order_id -> Text,
        zk_proof_hash -> Text,
        payment_tx_hash -> Text,
        status -> Text,
        created_at -> BigInt,
    }
}

diesel::table! {
    products (id) {
        id -> Integer,
        name -> Text,
        price_xmr -> Text,
        description -> Text,
        image_ipfs_hash -> Text,
        in_stock -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(orders, products,);
