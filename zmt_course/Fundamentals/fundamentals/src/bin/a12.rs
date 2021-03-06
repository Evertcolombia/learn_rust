// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics


// * Use an enum for the box color
enum Color {
    Brown,
    White,
    Yellow
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Color is Brown"),
            Color::White => println!("Color is White"),
            Color::Yellow => println!("Color is Yellow"),
        }
    }
}

struct Dimensions {
    height: f64,
    width: f64,
    dept: f64
}

impl Dimensions {
    fn print(&self) {
        println!("Dimensions = height: {:?}, widht: {:?}, dept: {:?}",
            self.height, self.width, self.dept);
    }
}

// * Use a struct to encapsulate the box characteristics
struct Box {
    color: Color,
    dimensions: Dimensions,
    weight: f64   
}


impl Box {
    // * Implement functionality on the box struct to create a new box
    fn create(color: Color, dimensions: Dimensions, weight: f64) -> Self {
        Self {
            color,
            dimensions,
            weight
        }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight);
    }
}

fn main() {

    let dimensions = Dimensions {
        height: 10.5,
        width: 6.6,
        dept: 12.0
    };

    let new_box = Box::create(Color::Yellow, dimensions, 15.5);
    new_box.print();

    let other_box = Box::create(
        Color::Brown,
        Dimensions {height: 5.6, width: 5.9, dept: 5.5},
        4.2
    );
    other_box.print()
}