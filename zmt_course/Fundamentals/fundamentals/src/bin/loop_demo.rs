fn main() {
    let mut i: i32 = 3;

    loop {
        

        println!("{:?}", i);
        i = i - 1;

        if i == 0 {
            break;
        }
    }
    println!("Done!");
}