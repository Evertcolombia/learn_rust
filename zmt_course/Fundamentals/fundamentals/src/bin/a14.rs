// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:

// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function


enum Color {
    Yellow,
    Pink,
    Red,
    Blue,
}

// * Use a struct for a persons age, name, and favorite color
struct Person {
    age: i32,
    name: String,
    favorite_color: Color,
}


// * The name and colors should be printed using a function
fn print_name(name: &str) {
    println!("Name: {:?}", name);
}

fn print_age(age: &i32) {
    println!("Age: {:?}", age);
}

fn print_color(color: &Color) {
    match color {
        Color::Yellow => println!("Favorite color is Yellow"),
        Color::Pink => println!("Favorite color is Pink"),
        Color::Red => println!("Favorite color is Red"),
        Color::Blue => println!("Favorite color is Blue"),
    }
}

fn main() {

    // * Create and store at least 3 people in a vector
    let persons = vec![
        Person {
            age: 5,
            name: "Thiago".to_owned(),
            favorite_color: Color::Blue,
        },
        Person {
            age: 9,
            name: String::from("Mateo"),
            favorite_color: Color::Red
        },
        Person {
            age: 55,
            name: String::from("Mercedes"),
            favorite_color: Color::Pink,
        },
        Person {
            age: 27,
            name: "Evert".to_owned(),
            favorite_color: Color::Yellow,
        }
    ];

    // * Iterate through the vector using a for..in loop
    for person in persons {
        // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            print_name(&person.name);
            print_color(&person.favorite_color);
            println!("\n");
        }
        else {
            print_name(&person.name);
            print_color(&person.favorite_color);
            print_age(&person.age);
            println!("\n");
        }
    }

}