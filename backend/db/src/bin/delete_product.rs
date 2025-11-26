use diesel::prelude::*;
use db::*;
use std::env::args;

fn main() {
    use self::schema::products::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(products.filter(name.like(pattern)))
        .execute(connection)
        .expect("Error deleting products");

    println!("Deleted {} products", num_deleted);
}