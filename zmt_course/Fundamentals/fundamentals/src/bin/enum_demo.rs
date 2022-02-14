enum Direction {
    Left,
    Right,
    Up,
}

fn main() {
    let go = Direction::Up;

    match go {
        Direction::Left => println!("go Left!"),
        Direction::Right => println!("go Right!"),
        Direction::Up => println!("go Up!"),
    }
}