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
    Yellow,
    White,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Color: Brown"),
            Color::Yellow => println!("Color: Yellow"),
            Color::White => println!("Color: White"),
        }
    }
}
// * Use a struct to encapsulate the box characteristics
struct Dimensions {
    height: f64,
    width: f64,
    dept: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Dimensions:");
        println!("Height: {:?} - Width: {:?} - Dept: {:?}",
            self.height, self.width, self.dept
        );
    }
}


struct ShippingBox {
    weight: f64,
    dimensions: Dimensions,
    color: Color,
}

impl ShippingBox {
    fn create(weight: f64, dimensions: Dimensions, color: Color) -> Self {
        Self {
            weight,
            dimensions,
            color,
        }
    }

    fn print(&self) {
        self.dimensions.print();
        self.color.print();
        println!("Weight: {:?}", self.weight);
    }
}

fn main() {
    let dimensions = Dimensions {
        height: 20.4,
        width: 12.5,
        dept: 13.4,
    };

    let new_box = ShippingBox::create(34.0, dimensions, Color::White);
    new_box.print();

    let other_box = ShippingBox::create(
        12.4,
        Dimensions { height: 15.4, width: 8.5, dept: 9.0 },
        Color::Brown
    );
    other_box.print();
}

