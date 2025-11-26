use self::models::Product;
use diesel::prelude::*;
use db::*;
use std::env::args;

fn main() {
    use self::schema::products::dsl::{products, in_stock};

    let id = args()
        .nth(1)
        .expect("store_product requires a product id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let product = diesel::update(products.find(id))
        .set(in_stock.eq(true))
        .returning(Product::as_returning())
        .get_result(connection)
        .unwrap();
    println!("Stored product {}", product.name);
}