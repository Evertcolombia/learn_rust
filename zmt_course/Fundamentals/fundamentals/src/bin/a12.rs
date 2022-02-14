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
    Red,
    Yellow,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Color is Red"),
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
        println!("Dimensions:\n Height: {:?} - Width {:?} - Dept: {:?}", 
            self.height,
            self.width,
            self.dept,
        );
    }
}

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}


impl ShippingBox {
    // * Implement functionality on the box struct to create a new box
    fn create(weight: f64, dimensions: Dimensions, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn print(&self) {
        println!("Weight: {:?}", self.weight);
        self.dimensions.print();
        self.color.print();
    }
}

fn main() {

    let dimensions = Dimensions {
        height: 34.5,
        width: 22.2,
        dept: 22.0,
    };

    let new_box = ShippingBox::create(12.2, dimensions, Color::Red);    
    new_box.print();
}