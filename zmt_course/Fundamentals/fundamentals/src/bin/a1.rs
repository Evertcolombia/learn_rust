// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn print_first_name(name: &str) {
    println!("{:?}", name);
}

fn print_last_name(last_name: &str) {
    println!("{:?}", last_name);
}

fn main() {
    let first_name = "Evert";
    let last_name = "Escalante";

    print_first_name(&first_name);
    print_last_name(&last_name);
}