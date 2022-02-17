// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result


// * Use a function to add two numbers together
fn add(a: i32, b: i32) -> i32  {
    a + b // the return line does not needs the ; semicolon
}

// * Use a function to display the result
fn display_result(result: i32) {
    // * Use the "{:?}" token in the println macro to display the result
    println!("The result is: {:?}", result); // as this function does not return we use ; semicolon
}


fn main() {
    let result = add(34, 43);
    display_result(result);
}