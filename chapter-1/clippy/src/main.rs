#![warn(clippy::all, clippy::pedantic)] // You can make clippy more pedantic and warn about more stuff with this
// BAD CODE
// fn main() {
//     let MYLIST = ["One", "Two", "Three"];
//     for i in 0..3 {
//         println!("{}", MYLIST[i]);
//     }
// }

// FIXED CODE
fn main() {
    let my_list = ["One", "Two", "Three"];
    for num in &my_list { // RUST supports iterators like python
        println!("{}", num);
    }
}
