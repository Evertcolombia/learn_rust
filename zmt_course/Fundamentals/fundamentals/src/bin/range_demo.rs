fn main() {
    // range between two values including the last one
    let range = 1..=3;

    // range between two values without including the last one
    let range2 = 4..8;

    for num in 10..15 {
        println!("{:?}", num);
    }

    for ch in 'a'..='f' {
        println!("{:?}", ch);
    }
}