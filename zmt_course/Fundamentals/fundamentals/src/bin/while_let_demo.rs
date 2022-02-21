fn main() {
    let mut data = Some(3);
    
    while let Some(i) = data {
        println!("loop");
        data = None;
    }

    let numbers = [1, 2, 3];
    let mut numbers_iter = numbers.iter();
    
    while let Some(num) = numbers_iter.next() {
        println!("num = {:?}", num);
    }

    println!("done!");
}