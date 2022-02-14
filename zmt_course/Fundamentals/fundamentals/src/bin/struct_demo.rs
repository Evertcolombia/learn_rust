struct GroceryItem {
    name: String,
    stock: i32,
    price: f64,
}

fn main() {
    let cereal = GroceryItem {
        name: String::from("Kellogs"),
        stock: 10,
        price: 12.50,
    };

    println!("Item name: {:?}", cereal.name);
    println!("Item stock: {:?}", cereal.stock);
    println!("Item price: {:?}", cereal.price);
}