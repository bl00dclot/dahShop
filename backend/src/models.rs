use diesel::prelude::*;
use crate::schema::products;
use serde::Serialize;


#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price_xmr: String,
    pub description: String,
    pub image_ipfs_hash: String,
    pub in_stock: bool,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::orders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Order {
    pub id: i32,
    pub order_id: String,
    zk_proof_hash: String,
    payment_tx_hash: String,
    status: String,
}

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub price_xmr: &'a str,
    pub description: &'a str,
    pub image_ipfs_hash: &'a str,

}