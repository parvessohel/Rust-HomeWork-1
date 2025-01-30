fn sh1kari(x: i32) {
    println!("The value of X is {}", x);
}

fn main() {
    let x;
    x = 5;
    sh1kari(x); // -> -> Compiler doesn't throw an error because `x` is used after being initialized
}
