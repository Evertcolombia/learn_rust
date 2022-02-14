fn main() {
    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    // destructuring a tuple values
    let (x, y) = (coord.0, coord.1);
    println!("{:?}, {:?}", x, y);

    let (name, age) = ("Evert", 27);
    println!("{:?}, {:?}", name, age);

    // when a tuple comes to big its better to use a struct to identify their values
    let favorites = ("red", 14, "Texas", "pizza", "Peaky Blinders", "Home");
    
    let state = favorites.2;
    let place = favorites.5;

    println!("{:?}, {:?}", state, place);
}