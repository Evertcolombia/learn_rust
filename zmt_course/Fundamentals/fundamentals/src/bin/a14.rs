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

struct Person {
    color: String,
    age: i32,
    name: String,
}

impl Person {
    fn print(&self) {
     println!("Name: {:?}\n Favorite Color: {:?}", self.name, self.color);
    }
}

fn main() {
    let persons = vec![
        Person {
            color: "Blue".to_owned(),
            age: 27,
            name: String::from("Evert Escalante")
        },
        Person {
            color: "Red".to_owned(),
            age: 21,
            name: String::from("Michael Ramirez")
        },
        Person {
            color: "Blue sky".to_owned(),
            age: 54,
            name: String::from("Mercedes Escalante")
        },
        Person {
            color: "Yellow".to_owned(),
            age: 10,
            name: String::from("Mateo Mendez")
        },
    ];

    for person in persons {
        if person.age > 10 {
            person.print();
        }
    }
}