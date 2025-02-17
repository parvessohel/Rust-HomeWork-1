fn idiot(x: i32) -> i32 {
    let original = x;
    let x = if original != 0 {
        original
    } else {
        original + 1
    };

    println!("Value of X is {}", x);
    x + 38 // The `if-else` statement is an expression in Rust it evaluates to a value.
}

fn main() {
    let x = 4;
    println!("Value of X is {}", x);
    let x = idiot(x);
    println!("Value of x is {}", x);
}
