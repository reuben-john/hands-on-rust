use std::io::stdin;

#[derive(Debug)] // This will allow you to print out the whole struct via a debug
struct Visitor {
    // This is a struct, like a class, that acts as a blueprint for something.
    name: String,
    greeting: String,
}

impl Visitor {
    // We create an implementation of the struct by creating a constructor
    // You define methods on the struct to interact
    // you need to use & to tell rust we are referencing the variable in memory

    fn new(name: &str, greeting: &str) -> Self {
        // Self refers to the struct while self refers to the instance

        // No semicolon says this is a return.  Every field must be included for it to work
        // This will take &str and return String since to_lowercase and to_string convert them.
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    // &self denotes you have access to the instance contents like self in a class
    fn greet_visitor(&self) {
        println!("{}", self.greeting) // You can debug your placeholder by using {:?}
    }
}

/// This will return a string
fn what_is_your_name() -> String {
    let mut your_name = String::new(); // Create new, empty string
    stdin()
        // Requires creating an empty string in order to save to it.
        // &mut indicates you are borrowing the variable in order to update it
        // prefixing with & creates a reference to a variable.  This allows you to pass the reference instead of a copy
        .read_line(&mut your_name)
        .expect("Failed to read line"); // Show this if it fails
    your_name // Return statement doesn't end in a semi colon
        .trim() // remove extra characters
        .to_lowercase() // convert to lowercase
}

fn main() {
    let visitor_list = [
        Visitor::new("bob", "Hello Bob, enjoy your visit."),
        Visitor::new("sally", "Hello Bob, Jane was asking about you."),
        Visitor::new("bender", "Hello Bender, the bar is fully stocked."), // the last trailing comma on a list is ignored
    ];
    println!("Hello, what's your name?");
    let your_name = what_is_your_name();
    // let mut allow_them_in = false;
    // for visitor in &visitor_list {
    //     // Loop through the visitor list
    //     if visitor == &your_name {
    //         allow_them_in = true;
    //     }
    // }
    //  This replaces the above loop. It uses iterators to replace the manual loop
    let known_visitor = visitor_list
        .iter() // Use iterators on the list
        .find(|visitor| visitor.name == your_name); // find runs a closure to determine what to look for. true if it finds it, false otherwise
        // This will set the matched item as the value to match inside the option
    println!("{:?}", known_visitor); // This will print out the full struct since it derives Debug
    // This will check for the value inside the Option returned by find
    // If found, assign it to visitor for inside the match
    // If None, do something else
    // => is used to mark code to execute for the match
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(), 
        None => println!("Sorry, you are not on the visitor list. Please leave.") // You can debug your placeholder by using {:?}
    }
    
}
