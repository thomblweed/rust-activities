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
    RED,
    BLUE,
    GREEN,
}

impl Color {
    fn print(&self) {
        match self {
            Color::RED => println!("Color: RED"),
            Color::BLUE => println!("Color: BLUE"),
            Color::GREEN => println!("Color: GREEN"),
        }
    }
}

struct Box {
    dimensions: i32,
    weight: i32,
    color: Color,
}

impl Box {
    fn new(dimensions: i32, weight: i32, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }
    fn print(&self) {
        println!("Dimensions: {:?}", self.dimensions);
        println!("Weight: {:?}", self.weight);
        self.color.print();
    }
}

fn main() {
    let red_box = Box::new(55, 23, Color::RED);
    red_box.print();
}
