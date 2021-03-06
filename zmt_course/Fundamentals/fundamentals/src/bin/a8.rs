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
enum Flavor {
    Cola,
    Apple,
    PinneApple,
    Cherry
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    ounces: f64,
}

// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: &Drink) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::Apple => println!("Flavor: Apple"),
        Flavor::Cola => println!("Flavor: Cola"),
        Flavor::Cherry => println!("Flavor: Cherry"),
        Flavor::PinneApple => println!("Flavor: Apple"),
    }

    println!("ounces: {:?}", drink.ounces);
}

fn main() {
    let my_drink = Drink {
        flavor: Flavor::Cola,
        ounces: 5.6
    };
    print_drink(&my_drink);
}

