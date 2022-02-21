// modules helps to keep your code self contain
// and easier to follow

mod greet {
    pub fn hello() {
        println!("Hello");
    }

    pub fn goodbye() {
        println!("Goodbye");
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }


    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    //use greet::hello;
    greet::hello();
    greet::goodbye();

    use math::*;
    println!("{:?}", add(23, 67));
    println!("{:?}", sub(23, 67)); 
}