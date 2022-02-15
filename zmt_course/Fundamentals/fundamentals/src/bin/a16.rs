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
// * The locker assignment should use an Option<i32>
struct Locker {
    name: String,
    number: Option<i32>,
}


fn main() {
    let student = Locker {
        name: String::from("Evert Escalante"),
        number: None, //Some(404),
    };

    println!("Name: {:?}", student.name);

    match student.number {
        Some(number) => println!("{:?}", number),
        None => println!("Not Locker Assigned"),
    }

}