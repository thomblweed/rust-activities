// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:

fn main() {
    // * Use a struct containing the student's name and locker assignment
    // * The locker assignment should use an Option<i32>
    struct Student {
        name: String,
        locker_assignment: Option<i32>,
    }

    impl Student {
        fn new(name: String, locker_assignment: Option<i32>) -> Self {
            Self {
                name,
                locker_assignment,
            }
        }
        fn print(self) {
            println!("Name: {:?}", self.name);
            match self.locker_assignment {
                Some(locker_number) => println!("Locker: {:?}", locker_number),
                None => println!("No locker assigned!"),
            }
        }
    }

    let students = vec![
        Student::new("thom".to_owned(), None),
        Student::new("jeff".to_owned(), Some(23)),
    ];

    for student in students {
        student.print();
    }
}
