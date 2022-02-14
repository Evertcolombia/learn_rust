// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {

    // Displays a message based on the value of a boolean variable
    //let display: bool = false;
    let display: bool = true;

    if display == true { // When the variable is set to true, display "hello"
        println!("hello");
    } else { // When the variable is set to false, display "goodbye"
        println!("goobye");
    }
}