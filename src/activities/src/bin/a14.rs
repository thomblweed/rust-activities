// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:

fn main() {
    // * Use a struct for a persons age, name, and favorite color
    // * The color and name should be stored as a String
    struct Person {
        age: i32,
        name: String,
        favourite_color: String,
    }
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            age: 12,
            name: String::from("John"),
            favourite_color: String::from("red"),
        },
        Person {
            age: 8,
            name: String::from("Jeff"),
            favourite_color: String::from("blue"),
        },
        Person {
            age: 5,
            name: String::from("Jenny"),
            favourite_color: String::from("green"),
        },
    ];
    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            print_name_colors(&person.name, &person.favourite_color);
        }
    }
}

// * The name and colors should be printed using a function
fn print_name_colors(name: &str, favourite_color: &str) {
    println!("{:?}", name);
    println!("{:?}", favourite_color);
}
