use std::cmp::max; // `use` is used to bring `max` into scope from the `std` crate's `cmp` module.
                   // use std::cmp::min; // This imports min
                   // use std::cmp::{max, min}; // We can also import like this.
                   // use std::{cmp::max, cmp::min}; // This also imports `max` and `min`, same as above.
                   // use std::cmp::*; This imports all items from the `cmp` module, including `min` and `max`.
fn main() {
    let smaller = std::cmp::min(4, 2); // Here std is a crate, cmp is a module, and min is a method. We can use those namespaces with double colon.
    println!("The Smaller number is {}", smaller);

    let bigger = max(4, 2);  // We can use max directly because we imported it with "use" keyword above.
    println!("The Bigger numer is {}", bigger);
}