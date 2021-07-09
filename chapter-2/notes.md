### chapter 2 notes


read_line is used to read input from the terminal

variables are all defined using let and are immutable by default
If you want it to be mutable, you must add mut to it

let name = 
let mut name = 

It is better to not use mutable if not required. It can add extra complexity

You import other packages or scripts using "use"

Instead of . like python, you use ::

use std::io::stdin will import stdin and allow access to that package

Rust supports function chaining like python. Formatting is different though. Chained functions are expected to each be on a different line.

Function chains will pass their returned value from one function to the next, top down.

Read line requires creating an empty string in order to save to it.
&mut indicates you are borrowing the variable in order to update it
prefixing with & creates a reference to a variable.  This allows you to pass the reference instead of a copy

println! allows you to add placeholders via {} like python. You can debug the placeholder by using {:?} which will give you raw data on it.

if/else works the same.  It uses && and || for "and" and "or" and ! for "not"

arrays in rust must be the same type and cannot change size

You can create an array using ["hi", "you"] 
Rust will infer the type of array to use based on what is inside
If you want to declare the type and length manually, you can use 
my_list : [&%str;3] = .... instead

rust has 2 types of strings

str is a string literal - these are created in source code and unchanged
String is a dynamic item because it stores details about location, length, etc. You can edit and append Strings.

For loops have a few ways of working

Ranges can be used in a for loop. Ranges do not include the last number.  0..10 is 0 to 9
You can use a range the uses the len() of the array.  0..my_list.len()
Best is iterating like python for x in &my_list

structs are like a class.  You define them using struct then use them using impl
