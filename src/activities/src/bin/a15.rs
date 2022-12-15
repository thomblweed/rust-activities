// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:

fn main() {
    // * Use an enum for the tickets with data associated with each variant
    enum Ticket {
        Backstage(String, f64),
        VIP(String, f64),
        Standard(f64),
    }
    // * Create one of each ticket and place into a vector
    let tickets: Vec<Ticket> = vec![
        Ticket::Backstage("Thomas".to_owned(), 5.55),
        Ticket::VIP("Dave".to_owned(), 10.99),
        Ticket::Standard(2.55),
    ];
    // * Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(name, price) => {
                println!("Backstage, Price: ${:?}, Name: {:?}", price, name)
            }
            Ticket::VIP(name, price) => {
                println!("VIP, Price: ${:?}, Name: {:?}", price, name)
            }
            Ticket::Standard(price) => println!("Standard, Price: ${:?}", price),
        }
    }
}
