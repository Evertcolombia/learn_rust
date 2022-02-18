// * A clousure is an anonymous function that you can create
// into your code

// * clousures are always defined into another function


fn main() {
    
    // * declare a clousure
    let add = |a, b| a + b;
    let sum  = add(2, 2);
    println!("{:?}", sum);
}