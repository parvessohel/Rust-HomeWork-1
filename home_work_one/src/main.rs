fn value_of_x(x: i32) -> i32 {
    println!("Value of x is {}", x);
    x
}

fn main() {
    let x = 4;

    let omega = value_of_x(x);
    println!("Value of X and Omega is {} and {}", x, omega);
    {
        let x = vec![4, 2, 0].iter().map(|x| x + 3).fold(0, |x, y| x + y);
        println!("The value of x is {}", x); // This x and the previous x are different. This x is inside of a curly braces. That mean it is inside a block of its own.
    }
}
