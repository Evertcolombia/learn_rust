// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print


// * Use an enum with color names as variants
enum Colors {
    Black,
    Red,
    Pink,
    Blue,
}


// * The function must use the enum as a parameter
fn color_printer(color: Colors) {
    // * Use a match expression to determine which color
    //   name to print

    match color {
        Colors::Black => println!("The color is Black"),
        Colors::Red => println!("The color is Red"),
        Colors::Pink => println!("The color is Pink"),
        Colors::Blue => println!("The color is Blue"),
    }
}

fn main() {
    //let color = Colors::Blue;
    // * Use a function to print the color name
    color_printer(Colors::Black);
}