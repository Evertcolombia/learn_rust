// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

// * Use a function to print the messages
fn print_message(result: bool) {
    // * Use a match expression to determine which message
    //   to print
    match result {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {

    // * Use a boolean variable set to an expression 
    //to store whether the value is > 100 or <= 100

    //let my_number = 100;
    let my_number = 120;

    let result: bool = my_number > 100; // --> with expression
    //let result: bool = if my_number > 100 { true } else { false };  ---> Without expression

    print_message(result);
}