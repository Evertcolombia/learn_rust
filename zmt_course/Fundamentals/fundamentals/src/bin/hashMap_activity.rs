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
    let mut stock = HashMap::new();

    stock.insert("Chairs".to_owned(), 5);
    stock.insert("Beds".to_owned(), 3);
    stock.insert("Tables".to_owned(), 2);
    stock.insert("Couches".to_owned(), 0);

    let mut total_stock = 0;
    for (item, quantity) in stock.iter() {
        total_stock = total_stock + quantity;
        let stock_count = if quantity == &0 {
            "out of stock!".to_owned()
        } else {
            format!("{:?}", quantity)
        };
        println!("Item: {:?} - stock = {:?}", item, stock_count);
    }

    println!("Total stock = {:?}", total_stock);
}
