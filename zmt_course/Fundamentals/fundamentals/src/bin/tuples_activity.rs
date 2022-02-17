// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

// * Use a function that returns a tuple
fn create_tuple() -> (i32, i32) {
    (23, 35)
}

fn main() {

    // * Destructure the return value into two variables
    let (x, y) = create_tuple();

    println!("{:?} is the value of x", x);

    // * Use an if..else if..else block to determine what to print
    if y > 5 {
        println!("y is greather than 5");
    } else if y < 5 {
        println!("y is less than 5");
    } else {
        println!("y is equal 5");
    }
}

