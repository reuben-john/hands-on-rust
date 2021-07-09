## Chapter 3

We will be creating a state machine to store our games state. This will allow us to save relevant details about the game.

Traits are a way to share functionality between objects.  They are kind of like interfaces.

Bracket-lib has a state for game state called GameState.  This requires the object implement tick().  


Results are the standard method for handling errors.  They are enumeration.

You can handle the errors via a match, an unwrap, or passing them to the parent

unwrap will crash if any errors occur
match statements and unwrap can make your code hard to read if they are all over.
The most readable way is to use the ? operator.  The only requirement is that the function must return a result type

bracket-lib uses the builder pattern which is a common rust pattern.  It uses the function chaining and separates options into smaller functions.

Rust has a special shorthand for match against a single case

if let Some(x) = variable_name {stuff}