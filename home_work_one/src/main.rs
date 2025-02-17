fn value_of_x(z: i32) ->i32{                  // This is a function that takes a parameter z of i32 and also returns a value of i32. The arrow indicates that teh function has a return type.
    println!("Value of x is {}", z);
    z // This function returns z and we dont neeed to to use the keyword "return" or semicolon.
}

fn main() {
    let x = 4;
    let y = 2;
    let z = x + y; // Semicolon marks the end of a statement.
    let omega = value_of_x(z);
    println!("Value of Omega is {}", omega);

    let x = vec![4, 2, 0]
    .iter()
    .map(|x| x + 3)
    .fold(0, |x, y| x + y); // This expression has multiple lines, but the semicolon at the end marks the end of the full statement.
    println!("The value of x is {}", x);
}
