// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>


// * Use a struct containing the student's name and locker assignment
struct Locker {
    name: String,
    number: Option<i32>
}

impl Locker {
    fn print(&self) {
        match self.number {
            Some(num) => println!("owner: {:?} - number: {:?}", self.name, num),
            None => println!("Owner: {:?} - number: not specified", self.name)
        }
    }
}

fn main() {

    let lockers = vec![
        Locker {
            name: "Mateo Mendez".to_owned(),
            number: Some(10)
        },
        Locker {
            name: String::from("Thiago Mendez"),
            number: None
        }
    ];

    for unit in lockers {
        unit.print();
    }
}