use std::io::stdin;

#[derive(Debug)] // This will allow you to print out the whole struct via a debug
struct Visitor {
    // This is a struct, like a class, that acts as a blueprint for something.
    name: String,
    action: VisitorAction, // enum
    age: i8,               // 8 bit signed integer.  Values between -127 and 127
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

impl Visitor {
    // We create an implementation of the struct by creating a constructor
    // You define methods on the struct to interact
    // you need to use & to tell rust we are referencing the variable in memory

    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        // Self refers to the struct while self refers to the instance

        // No semicolon says this is a return.  Every field must be included for it to work
        // This will take &str and return String since to_lowercase and to_string convert them.
        Self {
            name: name.to_lowercase(),
            action, // You can skip the variable is the same as the struct's field name
            age,
        }
    }
    // &self denotes you have access to the instance contents like self in a class
    fn greet_visitor(&self) {
        // println!("{}", self.greeting) // You can debug your placeholder by using {:?}

        // Match the action against the enum
        // Each match uses => for the code to execute on a match
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the bat cave, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                // This will destruture the data in the enum and make it available
                println!("Welcome to the bat cave, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
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
    // This converts the list to a vector which is generic and can expand as you add items to it
    // You must set it as mutable
    let mut visitor_list = vec![
        Visitor::new("bob", VisitorAction::Accept, 45), // call the enum action
        Visitor::new(
            "sally",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose-free milk is in the fridge"),
            },
            15,
        ),
        Visitor::new("bender", VisitorAction::Refuse, 30), // the last trailing comma on a list is ignored
    ];
    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();
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
            .find(|visitor| visitor.name == name); // find runs a closure to determine what to look for. true if it finds it, false otherwise
                                                   // This will set the matched item as the value to match inside the option
                                                   // println!("{:?}", known_visitor); // This will print out the full struct since it derives Debug
                                                   // This will check for the value inside the Option returned by find
                                                   // If found, assign it to visitor for inside the match
                                                   // If None, do something else
                                                   // => is used to mark code to execute for the match
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                // is_empty is a build in on String that checks for an empty string.  IE len == 0
                if name.is_empty() {
                    break; // Break out of loop()
                } else {
                    // Add new visitors instead
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list); // use # for pretty print
}
