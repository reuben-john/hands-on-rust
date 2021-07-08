### Chapter 1 notes


Defining a function in rust:

fn function_name() {}

fn main() will always run first no matter what order functions are in.

Rust has no garbage collection.  Everything inside the brackets is scoped and will be automatically removed upon completion.

println!() is how to print text to the console.  The ! marks a macro. Rust includes ! in the name to signify that it is a macro.

"Hello, World!" Is a string literal.  It supports unicode.


cargo clippy will give you details and warnings about your code

It will warn you of issues to improve your code quality.

You can make it include more warnings by adding "#![warn(clippy::all, clippy::pedantic)]" to the top of the rs file


There are a bunch of crates available (modules, packages) on crates.io that you can install and use.

You can search using cargo search

You can add dependencies by modifying the cargo toml.