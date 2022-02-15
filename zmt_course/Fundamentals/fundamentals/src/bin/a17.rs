// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {

    let my_string = "EVERT ESCALANTE";
    let my_string_2 = "ist a great night";
    println!("{:?}", my_string.to_lowercase());
    println!("{:?}", my_string_2.to_uppercase());

}