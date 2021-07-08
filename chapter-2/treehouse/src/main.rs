use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");
    let mut your_name = String::new(); // Create new, empty string
    stdin()
        // Requires creating an empty string in order to save to it.
        // &mut indicates you are borrowing the variable in order to update it
        // prefixing with & creates a reference to a variable.  This allows you to pass the reference instead of a copy
        .read_line(&mut your_name) 
        .expect("Failed to read line"); // Show this if it fails
    println!("Hello, {}", your_name)
}
