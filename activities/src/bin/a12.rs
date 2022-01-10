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

enum Color {
    Black,
    Red,
    Blue,
    Green,
}

impl Color {
    fn print_color(&self) {
        match self {
            Color::Black => println!("Color is Black"),
            Color::Red => println!("Color is Red"),
            Color::Blue => println!("Color is Blue"),
            Color::Green => println!("Color is Green"),
        }
    }
}

struct Dimensions {
    length: i32,
    breadth: i32,
    height: i32,
}

impl Dimensions {
    fn print_dimensions(&self) {
        println!("Length is {:?}", self.length);
        println!("Breadth is {:?}", self.breadth);
        println!("Height is {:?}", self.height);
    }
}

struct Crate {
    dimensions: Dimensions,
    weight: i32,
    color: Color,
}

impl Crate {
    fn new(dimensions: Dimensions, weight: i32, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }
    fn check_box(&self) {
        self.dimensions.print_dimensions();
        println!("Weight is {:?}", self.weight);
        self.color.print_color();
    }
}

fn main() {
    let dimensions = Dimensions {
        length: 20,
        breadth: 10,
        height: 10,
    };
    let some_crate = Crate::new(dimensions, 50, Color::Red);
    some_crate.check_box();
}
