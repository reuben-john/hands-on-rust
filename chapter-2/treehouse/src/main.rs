use std::io::stdin;


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
    let visitor_list = ["bob", "sally", "bender"];
    println!("Hello, what's your name?");
    let your_name = what_is_your_name();
    let mut allow_them_in = false;
    for visitor in &visitor_list { // Loop through the visitor list
        if visitor == &your_name {
            allow_them_in = true;
        }
    }

    if allow_them_in {
        println!("Hello, {}. Welcome to the cave of wonders..", your_name) // You can debug your placeholder by using {:?}    
    }
    else {
        println!("Sorry, {}. You are not on the list.", your_name) // You can debug your placeholder by using {:?}
    }
    
}