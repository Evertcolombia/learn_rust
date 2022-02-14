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
struct Item {
    // * Use two i32 fields for the quantity and id number
    id: i32,
    quantity: i32,
}

// * Create a function to display the quantity, with the struct as a parameter
fn print_quantity(item: &Item) {
    println!("Quantity: {:?}", item.quantity);
}

// * Create a function to display the id number, with the struct as a parameter
fn print_id(item: &Item) {
    println!("Id: {:?}", item.id);
}

fn main() {
    let tomaco = Item {
        id: 101,
        quantity: 404,
    };

    println!("Tomaco:");
    print_quantity(&tomaco);
    print_id(&tomaco);
}