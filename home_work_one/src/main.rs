fn main() {
    let x = 4;
    let y = 2;
    let z = x + y; // Semicolon marks the end of a statement.
    println!("Value of X is {}", z);

    let x = vec![4, 2, 0]
    .iter()
    .map(|x| x + 3)
    .fold(0, |x, y| x + y); // This expression has multiple lines, but the semicolon at the end marks the end of the full statement.
    println!("The value of x is {}", x);
}
