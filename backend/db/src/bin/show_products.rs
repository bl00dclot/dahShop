use self::models::*;
use diesel::prelude::*;
use db::*;

fn main() {
    use self::schema::products::dsl::*;

    let connection = &mut establish_connection();
    let results = products
        .filter(in_stock.eq(true))
        .limit(5)
        .select(Product::as_select())
        .load(connection)
        .expect("Error loading products");

    println!("Displaying {} products", results.len());
    for product in results {
        println!("{}", product.name);
        println!("-----------\n");
        println!("{}", product.description);
        println!("-----------\n");
        println!("{}", product.price_xmr);
    }
}