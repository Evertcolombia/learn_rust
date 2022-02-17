// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter


// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
struct GroceryItem {
    id: i32,
    quantity: i32
}

// * Create a function to display the id number, with the struct as a parameter
fn print_id(item: &GroceryItem) {
    println!("Item id: {:?}", item.id);
}

// * Create a function to display the quantity, with the struct as a parameter
fn print_quantity(item: &GroceryItem) {
    println!("Item quantity: {:?}", item.quantity);
}

fn main() {
    let new_item = GroceryItem {
        id: 1,
        quantity: 24
    };

    print_id(&new_item);
    print_quantity(&new_item);

}


