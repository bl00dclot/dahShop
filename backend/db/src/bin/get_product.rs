use self::models::Product;
use diesel::prelude::*;
use db::*;
use std::env::args;

fn main() {
    use self::schema::products::dsl::products;

    let product_id = args()
        .nth(1)
        .expect("get_products requires a product id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let product = products
        .find(product_id)
        .select(Product::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Post>, otherwise it will throw an error

    match product {
        Ok(Some(product)) => println!("Product with id: {} has a name: {}", product.id, product.name),
        Ok(None) => println!("Unable to find product {}", product_id),
        Err(_) => println!("An error occured while fetching product {}", product_id),
    }
}
