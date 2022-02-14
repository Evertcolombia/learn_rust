// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


// * Use an enum to create different flavors of drinks
enum DrinkFlavor {
    Coca,
    Hipinto,
    Pepsi,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: DrinkFlavor,
    ounces: f64,
}

// * Use a function to print out the drink flavor and ounces
fn print_flavor(drink: Drink) {

    // * Use a match expression to print the drink flavor
    match drink.flavor {
        DrinkFlavor::Coca => println!("Flavor: Coca cola"),
        DrinkFlavor::Hipinto => println!("Flavor: Hipinto"),
        DrinkFlavor::Pepsi => println!("Flavor: Pepsi cola"),
    }
    println!("Oz: {:?}\n", drink.ounces);
}


fn main() {
    let drink = Drink {
        flavor: DrinkFlavor::Pepsi,
        ounces: 10.0,
    };
    print_flavor(drink);

    let drink_2 = Drink {
        flavor: DrinkFlavor::Coca,
        ounces: 6.0,
    };
    print_flavor(drink_2);

    let drink_3 = Drink {
        flavor: DrinkFlavor::Hipinto,
        ounces: 30.5,
    };
    print_flavor(drink_3);
}