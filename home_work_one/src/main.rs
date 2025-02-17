fn main() {
    let x = 4;
    println!("Value of X is {}", x);
    let x = { 42 }; // Same as the first one.
    println!("Value of X is {}", x);

    let x = {
        // Curly braces in Rust define an expression block,
        let y = 4;
        let z = 2;
        y + z // allowing multiple operations like performing complex calculations, function calls, loops, and if-else blocks etc..
    };

    println!("Value of X is {}", x);
}
