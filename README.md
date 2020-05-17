# HashMap Implementation in Rust

The hash map uses separate chaining to deal with collisions.
The capacity of the map will double when it is half full.

The map also implements the IntoIterator, which allows it to be used in
rust's for loop syntax 

**Note: The goal of this was to get more familiar with rust, especially with traits, lifetimes, and rust move/borrow semantics.**
