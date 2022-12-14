// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:

fn main() {
    // * Use a vector to store 4 numbers
    let numbers = vec![10, 20, 30, 40];
    // assigning length of the vector to a variable so that we don't need to transfer numbers ownership to the for loop
    let numbers_length = numbers.len();

    // * Iterate through the vector using a for..in loop
    for number in numbers {
        // * Determine whether to print the number or print "thirty" inside the loop
        // if number == 30 {
        //     println!("thirty");
        //     break;
        // }
        // println!("{}", number);
        // alternative using match
        match number {
            30 => println!("thirty"),
            _ => println!("{:?}", number),
        }
    }
    // * Use the .len() function to print the number of elements in a vector
    println!("{}", numbers_length);
}
