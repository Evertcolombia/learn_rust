// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {
    println!("result: {:?}", clamp(50, 12, 70));
    println!("result: {:?}", clamp(7, 12, 70));

    println!("result: {:?}", concat("hello", "world"));
    println!("result: {:?}", div(45, 3));
    println!("result: {:?}", div(45, 0));

    
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_clamp_correct() {
        let result = clamp(50, 12, 70);
        let expected = 50;
        assert_eq!(result, expected, "N should be bigger than lower, and less than upper");
    }
    
    #[test]
    fn check_clamp_lower() {
        let result = clamp(7, 12, 70);
        let expected = 12;
        assert_eq!(result, expected, "should be 12");
    }

    #[test]
    
    fn check_div() {
        let result = div(45, 3);
        let expected = Some(15);
        assert_eq!(result, expected, "Division does not match ");
    }

    #[test]
    
    fn check_div_by_zero() {
        let result = div(45, 0);
        let expected =  None;
        assert_eq!(result, expected, "Division by zero is not valid");
    }

   #[test]
    fn check_concat() {
        let result = concat("hello", "world");
        let expected = "helloworld";
        assert_eq!(result, expected, "Concatenation string does not match");
    }

    #[test]
    fn check_concat_empty_arg() {
        let result = concat("", "world");
        let expected = "world";
        assert_eq!(result, expected, "Concatenation string does not match");
    }


}