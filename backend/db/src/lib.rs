use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::result::Error;
use self::models::{NewProduct, Product};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    // dotenvy::from_path("../");
// env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "./db/database.db";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn get_products_in_stock(conn: &mut SqliteConnection) -> Result<Vec<Product>, Error> {
    use crate::schema::products::dsl::*;

    products
        .filter(in_stock.eq(true))
        .select(Product::as_select())
        .load(conn)
}

pub fn create_product(conn: &mut SqliteConnection, name: &str, description: &str, price_xmr: &str, image_ipfs_hash: &str) -> Product {
    use crate::schema::products;

    let new_product = NewProduct {
        name,
        price_xmr,
        description,
        image_ipfs_hash
    };

    diesel::insert_into(products::table)
    .values(&new_product)
    .returning(Product::as_returning())
    .get_result(conn)
    .expect("Error storing product")
}