// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let furniture_store =
        HashMap::from([("Chairs", 5), ("Beds", 3), ("Tables", 2), ("Couches", 0)]);
    let total_number_items_in_stock: i32 = furniture_store.values().sum();

    for (item, number_in_stock) in furniture_store {
        if number_in_stock > 0 {
            println!("{:?} - {:?}", item, number_in_stock);
            continue;
        }
        println!("{:?} - out of stock", item);
    }
    println!(
        "Total number of items in stock: {:?}",
        total_number_items_in_stock
    );
}
