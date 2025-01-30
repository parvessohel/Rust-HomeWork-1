fn get_sh1kari(x: i32) {
    println!("The value of X is an Idiot {}", x);
}

fn main() {
    let x = 13;
    get_sh1kari(x);
    let x = x + 3;
    get_sh1kari(x); // -> Here this is the second one. As soon as a new variable is declared with the same name the earlier one is dropped and cannot be accessed. This is called Shadowing in Rust
                    // It is specailly useful if we want to change the type of the variable and we want to avoid declaring new variables with a different name to improve readability.
                    // Shadowing essentially lets you create a new variable with the same name.
}
