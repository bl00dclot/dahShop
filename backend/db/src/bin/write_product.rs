use db::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();
    let mut description = String::new();
    let price_xmr = String::from("123");
    let image_ipfs_hash = String::from("/images/product.webp");


    println!("Name of Product");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    println!("\nOk! Description of {name} (Press {EOF} when finished)\n",);
    stdin().read_to_string(&mut description).unwrap();

    let product = create_product(connection, &name, &description, &price_xmr, &image_ipfs_hash);
    println!("\nSaved draft {name} with id {}", product.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";