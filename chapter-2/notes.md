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

println! allows you to add placeholders via {} like python.


