// Topic: Result
//
// Requirements:

// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
#[derive(Debug)]
struct Adult {
    name: String,
    age: i32,
}

// * Implement a `new` function for the `Adult` structure that returns a Result:
// * The Ok variant should contain the initialized structure, but only
//   if the person is aged 21 or older
// * The Err variant should contain a String (or &str) that explains why
//   the structure could not be created
impl Adult {
    fn new(name: String, age: i32) -> Result<Self, String> {
        match age > 20 {
            true => Ok(Self { name, age }),
            false => Err(format!("{} must be 21 years of age or older", name)),
        }
    }
}

fn main() {
    // * Instantiate two `Adult` structures:
    // * One should be aged under 21
    // * One should be 21 or over
    let people = vec![
        Adult::new("Archie".to_owned(), 13),
        Adult::new("Thom".to_owned(), 41),
    ];

    // * Use `match` to print out a message for each `Adult`:
    // * For the Ok variant, print any message you want
    // * For the Err variant, print out the error message
    for person in people {
        print_person(person);
    }
}

fn print_person(person: Result<Adult, String>) {
    match person {
        Ok(adult) => println!(
            "{:?}, who is {:?} years old, was created successfully",
            adult.name, adult.age
        ),
        Err(error_message) => println!("{:?}", error_message),
    }
}

// fn learn() {
//     fn test(yo: bool) -> Result<String, String> {
//         match yo {
//             true => Ok("Yep".to_owned()),
//             false => Err("Nope".to_owned()),
//         }
//     }

//     fn clean_test(yo: bool) -> Result<String, String> {
//         let response = test(yo)?;
//         Ok(response)
//     }

//     let result = clean_test(false);
//     println!("{:?}", result);
// }
