fn sh1kari(x: i32) {
    println!("The value of X is {}", x);
}

fn main() {
    let x;
    sh1kari(x); // -> -> Compiler throws an error because `x` is used before being initialized
    x = 5;
}
